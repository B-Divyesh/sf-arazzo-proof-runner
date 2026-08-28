use arazzo_proof_runner::{
    RunOptions, RunStatus, compare_proofs, run_workflow, write_comparison, write_proof,
};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

fn write(path: &Path, content: &str) {
    fs::write(path, content).unwrap();
}

fn serve(responses: Vec<(&'static str, &'static str)>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    thread::spawn(move || {
        for (expected_path, body) in responses {
            let (mut stream, _) = listener.accept().unwrap();
            let request = read_request(&mut stream);
            let first_line = request.lines().next().unwrap_or_default();
            assert!(
                first_line.contains(expected_path),
                "expected {expected_path}, got {first_line}"
            );
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Fixture: stable\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
    });
    format!("http://{address}")
}

fn read_request(stream: &mut TcpStream) -> String {
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 2048];
    loop {
        let count = stream.read(&mut buffer).unwrap_or(0);
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
    String::from_utf8_lossy(&bytes).into_owned()
}

fn common_openapi(server: &str) -> String {
    format!(
        r#"openapi: 3.1.0
info: {{title: Fixture, version: 1.0.0}}
servers:
  - url: {server}
paths:
  /pets:
    post:
      operationId: createPet
      responses: {{'200': {{description: ok}}}}
  /pets/{{petId}}:
    get:
      operationId: getPet
      responses: {{'200': {{description: ok}}}}
  /health:
    get:
      operationId: health
      responses: {{'200': {{description: ok}}}}
  /version:
    get:
      operationId: version
      responses: {{'200': {{description: ok}}}}
"#
    )
}

fn env_file(server: &str) -> String {
    format!(
        r#"name: integration
baseUrl: {server}
inputs:
  petName: Ada
values:
  tenant: sandbox
headers:
  Authorization: Bearer token123
secrets:
  - token123
redact:
  - /owner/email
"#
    )
}

#[test]
// @claim:redaction-and-chaining
fn chained_workflow_produces_redacted_stable_evidence() {
    let server = serve(vec![
        (
            "POST /pets ",
            r#"{"id":"pet-7","name":"Ada","secret":"token123","owner":{"email":"ada@example.test"}}"#,
        ),
        (
            "GET /pets/pet-7?tenant=sandbox ",
            r#"{"id":"pet-7","state":"ready"}"#,
        ),
    ]);
    let temp = tempdir().unwrap();
    write(&temp.path().join("openapi.yaml"), &common_openapi(&server));
    write(&temp.path().join("env.yaml"), &env_file(&server));
    write(
        &temp.path().join("flow.yaml"),
        r#"arazzo: 1.0.1
info: {title: Chained, version: 1.0.0}
sourceDescriptions:
  - {name: api, url: ./openapi.yaml, type: openapi}
workflows:
  - workflowId: petLifecycle
    inputs:
      type: object
      required: [petName]
    steps:
      - stepId: create
        operationId: createPet
        requestBody:
          contentType: application/json
          payload: {name: '$inputs.petName'}
        successCriteria:
          - {condition: '$statusCode == 200'}
          - {condition: '$response.body#/name == "Ada"'}
        outputs:
          petId: '$response.body#/id'
      - stepId: fetch
        operationId: getPet
        parameters:
          - {name: petId, in: path, value: '$steps.create.outputs.petId'}
          - {name: tenant, in: query, value: '$env.tenant'}
        successCriteria:
          - {condition: '$response.body#/state == "ready"'}
"#,
    );
    let proof = run_workflow(&RunOptions {
        arazzo_path: temp.path().join("flow.yaml"),
        environment_path: temp.path().join("env.yaml"),
        workflow_id: None,
    })
    .unwrap();
    assert_eq!(proof.result, RunStatus::Passed);
    assert_eq!(proof.steps[1].outputs.len(), 0);
    let serialized = serde_json::to_string(&proof).unwrap();
    assert!(!serialized.contains("token123"));
    assert!(!serialized.contains("ada@example.test"));
    assert!(serialized.contains("[REDACTED]"));
    let output = temp.path().join("proof");
    write_proof(&proof, &output).unwrap();
    assert!(output.join("report.html").is_file());
}

#[test]
// @claim:operation-selection-and-parameters
fn operation_path_and_parameter_substitution_work() {
    let server = serve(vec![("GET /health?verbose=true ", r#"{"healthy":true}"#)]);
    let temp = tempdir().unwrap();
    let openapi: serde_json::Value =
        serde_yaml::from_str(&common_openapi(&server).replace("openapi: 3.1.0", "openapi: 3.0.3"))
            .unwrap();
    write(
        &temp.path().join("openapi.json"),
        &serde_json::to_string_pretty(&openapi).unwrap(),
    );
    let environment: serde_json::Value = serde_yaml::from_str(&env_file(&server)).unwrap();
    write(
        &temp.path().join("env.json"),
        &serde_json::to_string_pretty(&environment).unwrap(),
    );
    write(
        &temp.path().join("flow.yaml"),
        r#"arazzo: 1.0.0
info: {title: Health, version: 1.0.0}
sourceDescriptions:
  - {name: api, url: ./openapi.json, type: openapi}
workflows:
  - workflowId: healthReview
    steps:
      - stepId: health
        operationPath: '{$sourceDescriptions.api.url}#/paths/~1health/get'
        parameters:
          - {name: verbose, in: query, value: true}
          - {name: X-Tenant, in: header, value: '{$env.tenant}'}
          - {name: session, in: cookie, value: sample-cookie}
        successCriteria:
          - {condition: '$statusCode == 200'}
          - {condition: '$statusCode != 201'}
          - {condition: '$statusCode > 199'}
          - {condition: '$statusCode >= 200'}
          - {condition: '$statusCode < 201'}
          - {condition: '$statusCode <= 200'}
          - {condition: '$response.header.X-Fixture == "stable"'}
"#,
    );
    let proof = run_workflow(&RunOptions {
        arazzo_path: temp.path().join("flow.yaml"),
        environment_path: temp.path().join("env.json"),
        workflow_id: None,
    })
    .unwrap();
    assert_eq!(proof.result, RunStatus::Passed);
    assert_eq!(proof.steps[0].assertions.len(), 7);
    assert_eq!(proof.steps[0].request.headers["X-Tenant"], "sandbox");
    assert_eq!(proof.steps[0].request.headers["Cookie"], "[REDACTED]");
}

#[test]
// @claim:comparison-report
fn changed_response_assertion_is_visible_in_comparison_report() {
    let baseline_server = serve(vec![("GET /version ", r#"{"version":1}"#)]);
    let current_server = serve(vec![("GET /version ", r#"{"version":2}"#)]);
    let temp = tempdir().unwrap();
    write(
        &temp.path().join("openapi.yaml"),
        &common_openapi(&baseline_server),
    );
    write(
        &temp.path().join("baseline.env.yaml"),
        &env_file(&baseline_server),
    );
    write(
        &temp.path().join("current.env.yaml"),
        &env_file(&current_server),
    );
    write(
        &temp.path().join("flow.yaml"),
        r#"arazzo: 1.0.1
info: {title: Version, version: 1.0.0}
sourceDescriptions:
  - {name: api, url: ./openapi.yaml, type: openapi}
workflows:
  - workflowId: versionContract
    steps:
      - stepId: version
        operationId: version
        successCriteria:
          - {condition: '$response.body#/version == 1'}
"#,
    );
    let run = |env: &str| {
        run_workflow(&RunOptions {
            arazzo_path: temp.path().join("flow.yaml"),
            environment_path: temp.path().join(env),
            workflow_id: None,
        })
        .unwrap()
    };
    let baseline = run("baseline.env.yaml");
    let current = run("current.env.yaml");
    assert_eq!(baseline.result, RunStatus::Passed);
    assert_eq!(current.result, RunStatus::Failed);
    let comparison = compare_proofs(&baseline, &current);
    assert!(comparison.changed);
    assert!(
        comparison
            .changes
            .iter()
            .any(|change| change.field == "assertions")
    );
    let output = temp.path().join("comparison");
    write_comparison(&comparison, &output).unwrap();
    let html = fs::read_to_string(output.join("comparison.html")).unwrap();
    assert!(html.contains("assertions"));
    assert!(html.contains("false"));
}
