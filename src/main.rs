use anyhow::{Context, Result};
use arazzo_proof_runner::{
    ProofBundle, RunOptions, RunStatus, compare_proofs, run_workflow, write_comparison, write_proof,
};
use clap::{Parser, Subcommand};
use serde_json::json;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::ExitCode;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(name = "arazzo-proof", version, about = "Run an Arazzo workflow and keep the proof", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the bundled checkout workflow against an isolated local fixture.
    Demo {
        /// Print a machine-readable summary to stdout.
        #[arg(long)]
        json: bool,
    },
    /// Execute one workflow and write proof.json plus report.html.
    Run {
        /// Local Arazzo YAML or JSON document.
        workflow_file: PathBuf,
        /// Explicit YAML or JSON environment file. Never inferred.
        #[arg(long, value_name = "FILE")]
        env: PathBuf,
        /// Workflow ID. Required when the document contains more than one workflow.
        #[arg(long)]
        workflow: Option<String>,
        /// Output directory for proof.json and report.html.
        #[arg(long, default_value = "proof")]
        out: PathBuf,
        /// Print a machine-readable summary to stdout.
        #[arg(long)]
        json: bool,
    },
    /// Compare two proof.json files and write a review report.
    Compare {
        /// Baseline proof.json.
        baseline: PathBuf,
        /// Current proof.json.
        current: PathBuf,
        /// Output directory for comparison.json and comparison.html.
        #[arg(long, default_value = "proof-diff")]
        out: PathBuf,
        /// Print a machine-readable summary to stdout.
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match execute(cli) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::from(2)
        }
    }
}

fn execute(cli: Cli) -> Result<u8> {
    match cli.command {
        Command::Demo { json: machine } => run_demo(machine),
        Command::Run {
            workflow_file,
            env,
            workflow,
            out,
            json: machine,
        } => {
            let proof = run_workflow(&RunOptions {
                arazzo_path: workflow_file,
                environment_path: env,
                workflow_id: workflow,
            })?;
            write_proof(&proof, &out)?;
            if machine {
                println!(
                    "{}",
                    serde_json::to_string(&json!({
                        "result": proof.result,
                        "workflowId": proof.workflow_id,
                        "steps": proof.summary.steps,
                        "passed": proof.summary.passed,
                        "failed": proof.summary.failed,
                        "proof": out.join("proof.json"),
                        "report": out.join("report.html")
                    }))?
                );
            } else {
                println!(
                    "{}: {} steps, {} assertions passed, {} failed",
                    proof.workflow_id,
                    proof.summary.steps,
                    proof.summary.passed,
                    proof.summary.failed
                );
                println!(
                    "wrote {} and {}",
                    out.join("proof.json").display(),
                    out.join("report.html").display()
                );
            }
            Ok(if proof.result == RunStatus::Passed {
                0
            } else {
                1
            })
        }
        Command::Compare {
            baseline,
            current,
            out,
            json: machine,
        } => {
            let baseline_proof = read_proof(&baseline)?;
            let current_proof = read_proof(&current)?;
            let comparison = compare_proofs(&baseline_proof, &current_proof);
            write_comparison(&comparison, &out)?;
            if machine {
                println!(
                    "{}",
                    serde_json::to_string(&json!({
                        "changed": comparison.changed,
                        "added": comparison.summary.added,
                        "removed": comparison.summary.removed,
                        "modified": comparison.summary.modified,
                        "comparison": out.join("comparison.json"),
                        "report": out.join("comparison.html")
                    }))?
                );
            } else {
                println!(
                    "comparison: {}",
                    if comparison.changed {
                        "changed"
                    } else {
                        "unchanged"
                    }
                );
                println!(
                    "wrote {} and {}",
                    out.join("comparison.json").display(),
                    out.join("comparison.html").display()
                );
            }
            Ok(u8::from(comparison.changed))
        }
    }
}

fn run_demo(machine: bool) -> Result<u8> {
    let listener = TcpListener::bind("127.0.0.1:0").context("could not start demo fixture")?;
    let address = listener
        .local_addr()
        .context("could not read demo fixture address")?;
    let server = thread::spawn(move || serve_demo(listener));
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let workspace =
        std::env::temp_dir().join(format!("arazzo-proof-demo-{}-{nonce}", std::process::id()));
    fs::create_dir_all(&workspace)
        .with_context(|| format!("could not create demo workspace {}", workspace.display()))?;
    fs::write(
        workspace.join("checkout.arazzo.yaml"),
        include_str!("../examples/checkout.arazzo.yaml"),
    )?;
    fs::write(
        workspace.join("checkout.openapi.yaml"),
        include_str!("../examples/checkout.openapi.yaml"),
    )?;
    fs::write(
        workspace.join("demo.env.yaml"),
        format!(
            "name: bundled-demo\nbaseUrl: http://{address}\nheaders:\n  Authorization: Bearer demo-only-secret\nsecrets:\n  - demo-only-secret\n"
        ),
    )?;
    let output = workspace.join("proof");
    let proof = run_workflow(&RunOptions {
        arazzo_path: workspace.join("checkout.arazzo.yaml"),
        environment_path: workspace.join("demo.env.yaml"),
        workflow_id: Some("checkout".to_owned()),
    })?;
    write_proof(&proof, &output)?;
    server
        .join()
        .map_err(|_| anyhow::anyhow!("the demo fixture stopped unexpectedly"))??;
    if machine {
        println!(
            "{}",
            serde_json::to_string(&json!({
                "result": proof.result,
                "workflowId": proof.workflow_id,
                "steps": proof.summary.steps,
                "passed": proof.summary.passed,
                "failed": proof.summary.failed,
                "workspace": workspace,
                "proof": output.join("proof.json"),
                "report": output.join("report.html")
            }))?
        );
    } else {
        println!("Demo — bundled sample data, nothing is saved to your project.");
        println!(
            "{}: {} steps, {} assertions passed",
            proof.workflow_id, proof.summary.steps, proof.summary.passed
        );
        println!("report: {}", output.join("report.html").display());
        println!("proof:  {}", output.join("proof.json").display());
        println!(
            "workspace: {} (remove it when finished)",
            workspace.display()
        );
    }
    Ok(0)
}

fn serve_demo(listener: TcpListener) -> Result<()> {
    listener.set_nonblocking(false)?;
    for _ in 0..3 {
        let (mut stream, _) = listener.accept()?;
        let request = read_demo_request(&mut stream)?;
        let line = request.lines().next().unwrap_or_default();
        let (status, body) = if line.starts_with("POST /carts ") {
            (
                201,
                r#"{"id":"crt_17","owner":{"email":"demo@example.test"}}"#,
            )
        } else if line.starts_with("POST /carts/crt_17/items ") {
            (200, r#"{"itemCount":1,"accepted":true}"#)
        } else if line.starts_with("GET /carts/crt_17/quote ") {
            (200, r#"{"total":42,"currency":"USD"}"#)
        } else {
            (404, r#"{"error":"unknown demo route"}"#)
        };
        let reason = if status == 404 { "Not Found" } else { "OK" };
        write!(
            stream,
            "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nX-Demo-Fixture: bundled\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )?;
    }
    Ok(())
}

fn read_demo_request(stream: &mut TcpStream) -> Result<String> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 2048];
    loop {
        let count = stream.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..count]);
        if let Some(header_end) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
            let headers = String::from_utf8_lossy(&bytes[..header_end]);
            let length = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().ok())
                        .flatten()
                })
                .unwrap_or(0);
            if bytes.len() >= header_end + 4 + length {
                break;
            }
        }
    }
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn read_proof(path: &PathBuf) -> Result<ProofBundle> {
    let text =
        fs::read_to_string(path).with_context(|| format!("could not read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("invalid proof JSON in {}", path.display()))
}
