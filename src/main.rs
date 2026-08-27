use anyhow::{Context, Result};
use arazzo_proof_runner::{
    ProofBundle, RunOptions, RunStatus, compare_proofs, run_workflow, write_comparison, write_proof,
};
use clap::{Parser, Subcommand};
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "arazzo-proof", version, about = "Run an Arazzo workflow and keep the proof", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
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

fn read_proof(path: &PathBuf) -> Result<ProofBundle> {
    let text =
        fs::read_to_string(path).with_context(|| format!("could not read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("invalid proof JSON in {}", path.display()))
}
