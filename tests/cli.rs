use std::process::Command;
use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    thread,
};
use tempfile::tempdir;

#[test]
fn help_documents_non_interactive_commands() {
    let output = Command::new(env!("CARGO_BIN_EXE_arazzo-proof"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("run"));
    assert!(help.contains("compare"));
    assert!(help.contains("Run an Arazzo workflow and keep the proof"));
}

#[test]
fn run_command_writes_bundle_and_returns_assertion_exit_code() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let server = format!("http://{}", listener.local_addr().unwrap());
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = [0_u8; 1024];
        let _ = stream.read(&mut buffer);
        let body = r#"{"ready":false}"#;
        write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
    });
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("openapi.yaml"),
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
        temp.path().join("flow.yaml"),
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
        temp.path().join("env.yaml"),
        format!("name: cli\nbaseUrl: {server}\n"),
    )
    .unwrap();
    let output_dir = temp.path().join("proof");
    let output = Command::new(env!("CARGO_BIN_EXE_arazzo-proof"))
        .args([
            "run",
            temp.path().join("flow.yaml").to_str().unwrap(),
            "--env",
            temp.path().join("env.yaml").to_str().unwrap(),
            "--out",
            output_dir.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(summary["failed"], 1);
    assert!(output_dir.join("proof.json").is_file());
    assert!(output_dir.join("report.html").is_file());
}
