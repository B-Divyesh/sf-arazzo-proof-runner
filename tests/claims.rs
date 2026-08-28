use arazzo_proof_runner::{RunOptions, run_workflow};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[test]
// @claim:bundled-demo
fn claim_bundled_demo_is_isolated_and_writes_real_proof() {
    let mut workspaces = Vec::new();
    for _ in 0..2 {
        let output = Command::new(env!("CARGO_BIN_EXE_arazzo-proof"))
            .args(["demo", "--json"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(summary["workflowId"], "checkout");
        assert_eq!(summary["steps"], 3);
        assert_eq!(summary["passed"], 3);
        let workspace = Path::new(summary["workspace"].as_str().unwrap());
        assert_eq!(workspace.parent(), Some(std::env::temp_dir().as_path()));
        assert!(
            workspace
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("arazzo-proof-demo-")
        );
        let proof_path = Path::new(summary["proof"].as_str().unwrap());
        let report_path = Path::new(summary["report"].as_str().unwrap());
        assert!(proof_path.starts_with(workspace) && report_path.starts_with(workspace));
        let proof = fs::read_to_string(proof_path).unwrap();
        let report = fs::read_to_string(report_path).unwrap();
        assert!(proof.contains("createCart") && proof.contains("quoteCart"));
        assert!(report.contains("createCart") && report.contains("quoteCart"));
        assert!(!proof.contains("demo-only-secret"));
        assert!(!report.contains("demo-only-secret"));
        assert_html_is_self_contained(&report);
        workspaces.push(workspace.to_path_buf());
    }
    assert_ne!(
        workspaces[0], workspaces[1],
        "demo workspaces must be unique"
    );
    for workspace in workspaces {
        fs::remove_dir_all(workspace).unwrap();
    }
}

fn assert_html_is_self_contained(html: &str) {
    for attribute in ["src=\"", "href=\""] {
        let mut remainder = html;
        while let Some(start) = remainder.find(attribute) {
            remainder = &remainder[start + attribute.len()..];
            let end = remainder.find('"').expect("unterminated HTML attribute");
            let value = &remainder[..end];
            assert!(
                value.is_empty() || value.starts_with('#') || value.starts_with("data:"),
                "report contains a fetchable asset reference: {attribute}{value:?}"
            );
            remainder = &remainder[end + 1..];
        }
    }
}

#[test]
// @claim:single-binary-mit
fn claim_package_declares_one_binary_and_mit_license() {
    let manifest = fs::read_to_string("Cargo.toml").unwrap();
    assert_eq!(manifest.matches("[[bin]]").count(), 1);
    assert!(manifest.contains("name = \"arazzo-proof\""));
    assert!(manifest.contains("version = \"0.1.0\""));
    assert!(manifest.contains("license = \"MIT\""));
    let license = fs::read_to_string("LICENSE").unwrap();
    assert!(license.contains("Permission is hereby granted, free of charge"));
}

fn write_minimal_files(root: &Path, extra_step: &str, openapi_extra: &str) {
    fs::write(
        root.join("openapi.yaml"),
        format!(
            "openapi: 3.1.0\ninfo: {{title: Test, version: 1.0.0}}\npaths:\n  /ok:\n    get:\n      operationId: ok\n      responses: {{'200': {{description: ok}}}}\n{openapi_extra}"
        ),
    )
    .unwrap();
    fs::write(
        root.join("env.yaml"),
        "name: test\nbaseUrl: http://127.0.0.1:9\n",
    )
    .unwrap();
    fs::write(
        root.join("flow.yaml"),
        format!(
            "arazzo: 1.0.1\ninfo: {{title: Test, version: 1.0.0}}\nsourceDescriptions: [{{name: api, url: ./openapi.yaml, type: openapi}}]\nworkflows:\n  - workflowId: test\n    steps:\n      - stepId: check\n        operationId: ok\n{extra_step}"
        ),
    )
    .unwrap();
}

#[test]
// @claim:unsupported-errors
fn claim_unsupported_features_name_the_problem_before_requests() {
    let cases = [
        (
            "        onSuccess: [{name: next}]\n",
            "",
            "onSuccess/onFailure",
        ),
        (
            "        successCriteria: [{condition: '$statusCode == 200', context: '$.body'}]\n",
            "",
            "JSONPath",
        ),
        ("        retryAfter: 2\n", "", "retry policy"),
        (
            "        requestBody: {contentType: text/plain, payload: hi}\n",
            "",
            "request content type",
        ),
        ("", "callbacks: {}\n", "callbacks"),
        ("", "webhooks: {}\n", "webhooks"),
        (
            "",
            "components: {schemas: {X: {$ref: './other.yaml'}}}\n",
            "external $ref",
        ),
        (
            "",
            "components: {securitySchemes: {login: {type: oauth2, flows: {}}}}\n",
            "OAuth flow",
        ),
    ];
    for (step, source, expected) in cases {
        let temp = tempdir().unwrap();
        write_minimal_files(temp.path(), step, source);
        let error = run_workflow(&RunOptions {
            arazzo_path: temp.path().join("flow.yaml"),
            environment_path: temp.path().join("env.yaml"),
            workflow_id: None,
        })
        .unwrap_err()
        .to_string();
        assert!(
            error.contains(expected),
            "expected {expected:?} in {error:?}"
        );
        assert!(!temp.path().join("proof").exists());
    }

    let temp = tempdir().unwrap();
    fs::write(temp.path().join("env.yaml"), "name: test\n").unwrap();
    fs::write(
        temp.path().join("flow.yaml"),
        "arazzo: 1.0.1\nsourceDescriptions: [{name: api, url: https://example.test/openapi.yaml}]\nworkflows: [{workflowId: test, steps: [{stepId: check, operationId: ok}]}]\n",
    )
    .unwrap();
    let error = run_workflow(&RunOptions {
        arazzo_path: temp.path().join("flow.yaml"),
        environment_path: temp.path().join("env.yaml"),
        workflow_id: None,
    })
    .unwrap_err()
    .to_string();
    assert!(error.contains("remote sourceDescription"));
}
