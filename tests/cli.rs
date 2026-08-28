use std::ffi::OsStr;
use std::process::{Command, Output, Stdio};
use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::{Duration, Instant},
};
use tempfile::tempdir;

#[test]
// @claim:cli-help
fn help_documents_non_interactive_commands() {
    let output = Command::new(env!("CARGO_BIN_EXE_arazzo-proof"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("run"));
    assert!(help.contains("compare"));
    assert!(help.contains("demo"));
    assert!(help.contains("Run an Arazzo workflow and save a proof bundle"));
}

#[test]
// @claim:exit-codes-and-json
fn claim_exit_codes_and_json_cover_run_and_compare_matrix() {
    let temp = tempdir().unwrap();
    let passing = run_fixture(temp.path(), "passing", true);
    assert_eq!(passing.status.code(), Some(0));
    let passing_summary: serde_json::Value = serde_json::from_slice(&passing.stdout).unwrap();
    assert_eq!(passing_summary["result"], "passed");

    let failing = run_fixture(temp.path(), "failing", false);
    assert_eq!(failing.status.code(), Some(1));
    let failing_summary: serde_json::Value = serde_json::from_slice(&failing.stdout).unwrap();
    assert_eq!(failing_summary["result"], "failed");
    assert_eq!(failing_summary["failed"], 1);

    let invalid_run = run_cli([
        OsStr::new("run"),
        temp.path().join("missing-flow.yaml").as_os_str(),
        OsStr::new("--env"),
        temp.path().join("missing-env.yaml").as_os_str(),
        OsStr::new("--json"),
    ]);
    assert_eq!(invalid_run.status.code(), Some(2));
    assert!(invalid_run.stdout.is_empty());

    let passing_proof = temp.path().join("passing-proof/proof.json");
    let failing_proof = temp.path().join("failing-proof/proof.json");
    let unchanged = run_cli([
        OsStr::new("compare"),
        passing_proof.as_os_str(),
        passing_proof.as_os_str(),
        OsStr::new("--out"),
        temp.path().join("unchanged").as_os_str(),
        OsStr::new("--json"),
    ]);
    assert_eq!(unchanged.status.code(), Some(0));
    let unchanged_summary: serde_json::Value = serde_json::from_slice(&unchanged.stdout).unwrap();
    assert_eq!(unchanged_summary["changed"], false);

    let changed = run_cli([
        OsStr::new("compare"),
        passing_proof.as_os_str(),
        failing_proof.as_os_str(),
        OsStr::new("--out"),
        temp.path().join("changed").as_os_str(),
        OsStr::new("--json"),
    ]);
    assert_eq!(changed.status.code(), Some(1));
    let changed_summary: serde_json::Value = serde_json::from_slice(&changed.stdout).unwrap();
    assert_eq!(changed_summary["changed"], true);

    let invalid_compare = run_cli([
        OsStr::new("compare"),
        passing_proof.as_os_str(),
        temp.path().join("missing-proof.json").as_os_str(),
        OsStr::new("--out"),
        temp.path().join("invalid").as_os_str(),
        OsStr::new("--json"),
    ]);
    assert_eq!(invalid_compare.status.code(), Some(2));
    assert!(invalid_compare.stdout.is_empty());
}

fn run_fixture(root: &std::path::Path, name: &str, ready: bool) -> Output {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let server = format!("http://{}", listener.local_addr().unwrap());
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = [0_u8; 1024];
        let _ = stream.read(&mut buffer);
        let body = format!(r#"{{"ready":{ready}}}"#);
        write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
    });
    let fixture = root.join(name);
    fs::create_dir_all(&fixture).unwrap();
    fs::write(
        fixture.join("openapi.yaml"),
        format!(
            r#"openapi: 3.1.0
info: {{title: CLI, version: 1.0.0}}
servers: [{{url: {server}}}]
paths:
  /ready:
    get:
      operationId: ready
      responses: {{'200': {{description: ok}}}}
"#
        ),
    )
    .unwrap();
    fs::write(
        fixture.join("flow.yaml"),
        r#"arazzo: 1.0.1
info: {title: CLI, version: 1.0.0}
sourceDescriptions: [{name: api, url: ./openapi.yaml, type: openapi}]
workflows:
  - workflowId: cliProof
    steps:
      - stepId: ready
        operationId: ready
        successCriteria:
          - {condition: '$response.body#/ready == true'}
"#,
    )
    .unwrap();
    fs::write(
        fixture.join("env.yaml"),
        format!("name: cli\nbaseUrl: {server}\n"),
    )
    .unwrap();
    let output_dir = root.join(format!("{name}-proof"));
    let output = run_cli([
        OsStr::new("run"),
        fixture.join("flow.yaml").as_os_str(),
        OsStr::new("--env"),
        fixture.join("env.yaml").as_os_str(),
        OsStr::new("--out"),
        output_dir.as_os_str(),
        OsStr::new("--json"),
    ]);
    assert!(output_dir.join("proof.json").is_file());
    assert!(output_dir.join("report.html").is_file());
    output
}

fn run_cli<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut child = Command::new(env!("CARGO_BIN_EXE_arazzo-proof"))
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if child.try_wait().unwrap().is_some() {
            return child.wait_with_output().unwrap();
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            panic!("CLI waited for input or exceeded ten seconds");
        }
        thread::sleep(Duration::from_millis(10));
    }
}
