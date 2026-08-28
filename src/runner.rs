use crate::model::{
    AssertionEvidence, Environment, ProofBundle, ProofSummary, RequestEvidence, ResponseEvidence,
    RunStatus, StepEvidence,
};
use crate::parse::{
    ArazzoDocument, OpenApiSource, Step, Workflow, load_sources, read_yaml_or_json,
};
use crate::redact::{redact_headers, redact_json, redact_string, secret_values};
use crate::report::render_proof;
use anyhow::{Context, Result, anyhow, bail};
use reqwest::Method;
use reqwest::blocking::Client;
use serde_json::{Map, Number, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Options for one deterministic workflow execution.
#[derive(Clone, Debug)]
pub struct RunOptions {
    pub arazzo_path: PathBuf,
    pub environment_path: PathBuf,
    pub workflow_id: Option<String>,
}

#[derive(Default)]
struct EvalContext {
    inputs: BTreeMap<String, Value>,
    env: BTreeMap<String, Value>,
    step_outputs: BTreeMap<String, BTreeMap<String, Value>>,
}

struct ResponseContext {
    status: u16,
    headers: BTreeMap<String, String>,
    body: Option<Value>,
}

struct Operation<'a> {
    source: &'a OpenApiSource,
    method: String,
    path: String,
    value: &'a Value,
}

/// Execute a workflow and return only redacted, serialization-safe evidence.
pub fn run_workflow(options: &RunOptions) -> Result<ProofBundle> {
    let doc: ArazzoDocument = read_yaml_or_json(&options.arazzo_path)?;
    if !doc.arazzo.starts_with("1.0") {
        bail!(
            "unsupported Arazzo version '{}'; this runner supports the 1.0.x subset",
            doc.arazzo
        );
    }
    let environment: Environment = read_yaml_or_json(&options.environment_path)?;
    if environment.name.trim().is_empty() {
        bail!("environment name must not be empty");
    }
    let workflow = select_workflow(&doc.workflows, options.workflow_id.as_deref())?;
    validate_workflow(workflow)?;
    let sources = load_sources(&doc, &options.arazzo_path)?;
    let inputs = workflow_inputs(workflow, &environment)?;
    let mut context = EvalContext {
        inputs,
        env: environment.values.clone(),
        step_outputs: BTreeMap::new(),
    };
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent(concat!("arazzo-proof/", env!("CARGO_PKG_VERSION")))
        .build()
        .context("could not create HTTP client")?;
    let secrets = secret_values(&environment.headers, &environment.secrets);
    let mut evidence = Vec::with_capacity(workflow.steps.len());

    for step in &workflow.steps {
        let operation = find_operation(step, &sources)?;
        let step_evidence = execute_step(
            &client,
            step,
            &operation,
            &environment,
            &mut context,
            &secrets,
        )?;
        evidence.push(step_evidence);
    }

    let assertions = evidence.iter().map(|step| step.assertions.len()).sum();
    let passed = evidence
        .iter()
        .flat_map(|step| &step.assertions)
        .filter(|a| a.passed)
        .count();
    let failed = assertions - passed;
    let result = if failed == 0 {
        RunStatus::Passed
    } else {
        RunStatus::Failed
    };
    let source_file = options
        .arazzo_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("workflow.yaml")
        .to_owned();

    Ok(ProofBundle {
        schema_version: "arazzo-proof/v1".to_owned(),
        arazzo_version: doc.arazzo,
        workflow_id: workflow.workflow_id.clone(),
        environment: environment.name,
        source_file,
        result,
        summary: ProofSummary {
            steps: evidence.len(),
            assertions,
            passed,
            failed,
        },
        steps: evidence,
    })
}

/// Write `proof.json` and a fully self-contained `report.html` to a directory.
pub fn write_proof(proof: &ProofBundle, output: &Path) -> Result<()> {
    fs::create_dir_all(output).with_context(|| format!("could not create {}", output.display()))?;
    let json = serde_json::to_string_pretty(proof).context("could not serialize proof")?;
    fs::write(output.join("proof.json"), format!("{json}\n"))
        .with_context(|| format!("could not write {}/proof.json", output.display()))?;
    fs::write(output.join("report.html"), render_proof(proof))
        .with_context(|| format!("could not write {}/report.html", output.display()))?;
    Ok(())
}

fn select_workflow<'a>(workflows: &'a [Workflow], requested: Option<&str>) -> Result<&'a Workflow> {
    if let Some(id) = requested {
        return workflows
            .iter()
            .find(|workflow| workflow.workflow_id == id)
            .ok_or_else(|| anyhow!("workflow '{}' was not found", id));
    }
    match workflows {
        [only] => Ok(only),
        [] => bail!("the Arazzo document contains no workflows"),
        _ => bail!("the Arazzo document contains multiple workflows; select one with --workflow"),
    }
}

fn validate_workflow(workflow: &Workflow) -> Result<()> {
    if workflow.steps.is_empty() {
        bail!("workflow '{}' has no steps", workflow.workflow_id);
    }
    for step in &workflow.steps {
        if step.operation_id.is_some() == step.operation_path.is_some() {
            bail!(
                "step '{}' must define exactly one of operationId or operationPath",
                step.step_id
            );
        }
        if !step.on_success.is_empty() || !step.on_failure.is_empty() {
            bail!(
                "step '{}' uses onSuccess/onFailure actions, which are unsupported",
                step.step_id
            );
        }
        if step.retry_after.is_some() {
            bail!(
                "step '{}' uses a retry policy, which is unsupported",
                step.step_id
            );
        }
        if step
            .success_criteria
            .iter()
            .any(|criterion| criterion.context.is_some())
        {
            bail!(
                "step '{}' uses criterion context/JSONPath, which is unsupported",
                step.step_id
            );
        }
    }
    Ok(())
}

fn workflow_inputs(
    workflow: &Workflow,
    environment: &Environment,
) -> Result<BTreeMap<String, Value>> {
    let mut inputs = environment.inputs.clone();
    if let Some(schema) = &workflow.inputs {
        if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
            for (name, property) in properties {
                if !inputs.contains_key(name)
                    && let Some(default) = property.get("default")
                {
                    inputs.insert(name.clone(), default.clone());
                }
            }
        }
        if let Some(required) = schema.get("required").and_then(Value::as_array) {
            for name in required.iter().filter_map(Value::as_str) {
                if !inputs.contains_key(name) {
                    bail!(
                        "required workflow input '{}' is missing from the environment file",
                        name
                    );
                }
            }
        }
    }
    Ok(inputs)
}

fn find_operation<'a>(step: &Step, sources: &'a [OpenApiSource]) -> Result<Operation<'a>> {
    if let Some(operation_id) = &step.operation_id {
        let mut matches = Vec::new();
        for source in sources {
            let Some(paths) = source.document.get("paths").and_then(Value::as_object) else {
                continue;
            };
            for (path, item) in paths {
                for method in http_methods() {
                    if let Some(operation) = item.get(method)
                        && operation.get("operationId").and_then(Value::as_str)
                            == Some(operation_id)
                    {
                        matches.push(Operation {
                            source,
                            method: method.to_uppercase(),
                            path: path.clone(),
                            value: operation,
                        });
                    }
                }
            }
        }
        return match matches.len() {
            1 => Ok(matches.remove(0)),
            0 => bail!(
                "operationId '{}' from step '{}' was not found",
                operation_id,
                step.step_id
            ),
            _ => bail!(
                "operationId '{}' is ambiguous across sourceDescriptions",
                operation_id
            ),
        };
    }

    let raw = step
        .operation_path
        .as_deref()
        .expect("validated operation path");
    let (source_name, pointer) = parse_operation_path(raw)?;
    let source = if let Some(name) = source_name {
        sources
            .iter()
            .find(|source| source.name == name)
            .ok_or_else(|| {
                anyhow!(
                    "operationPath references unknown sourceDescription '{}'",
                    name
                )
            })?
    } else if sources.len() == 1 {
        &sources[0]
    } else {
        bail!(
            "operationPath '{}' must name a sourceDescription when multiple sources exist",
            raw
        );
    };
    let operation = source.document.pointer(pointer).ok_or_else(|| {
        anyhow!(
            "operationPath '{}' does not exist in {}",
            raw,
            source.path.display()
        )
    })?;
    let parts: Vec<&str> = pointer.split('/').collect();
    if parts.len() < 4 || parts[1] != "paths" {
        bail!(
            "operationPath '{}' must point to /paths/<path>/<method>",
            raw
        );
    }
    let method = parts.last().expect("parts").to_ascii_uppercase();
    if !http_methods().contains(&method.to_ascii_lowercase().as_str()) {
        bail!("operationPath '{}' points to unsupported HTTP method", raw);
    }
    let encoded_path = parts[2];
    let path = encoded_path.replace("~1", "/").replace("~0", "~");
    Ok(Operation {
        source,
        method,
        path,
        value: operation,
    })
}

fn parse_operation_path(raw: &str) -> Result<(Option<&str>, &str)> {
    if let Some(pointer) = raw.strip_prefix('#') {
        return Ok((None, pointer));
    }
    let prefix = "{$sourceDescriptions.";
    let Some(rest) = raw.strip_prefix(prefix) else {
        bail!(
            "unsupported operationPath '{}'; use a local JSON Pointer",
            raw
        );
    };
    let Some((name, pointer)) = rest.split_once(".url}#") else {
        bail!("invalid sourceDescription operationPath '{}'", raw);
    };
    Ok((Some(name), pointer))
}

fn http_methods() -> &'static [&'static str] {
    &[
        "get", "post", "put", "patch", "delete", "head", "options", "trace",
    ]
}

fn execute_step(
    client: &Client,
    step: &Step,
    operation: &Operation<'_>,
    environment: &Environment,
    context: &mut EvalContext,
    secrets: &[String],
) -> Result<StepEvidence> {
    let base = environment
        .base_url
        .as_deref()
        .or_else(|| {
            operation
                .value
                .get("servers")
                .and_then(Value::as_array)
                .and_then(|s| s.first())
                .and_then(|s| s.get("url"))
                .and_then(Value::as_str)
        })
        .or_else(|| {
            operation
                .source
                .document
                .get("servers")
                .and_then(Value::as_array)
                .and_then(|s| s.first())
                .and_then(|s| s.get("url"))
                .and_then(Value::as_str)
        })
        .ok_or_else(|| {
            anyhow!(
                "step '{}' has no base URL; set environment baseUrl or an OpenAPI server",
                step.step_id
            )
        })?;
    if base.contains('{') {
        bail!("templated OpenAPI server URLs are unsupported; set a concrete environment baseUrl");
    }
    let mut path = operation.path.clone();
    let mut query = Vec::<(String, String)>::new();
    let mut headers = environment.headers.clone();
    let mut cookies = Vec::new();

    for parameter in &step.parameters {
        let value = resolve_value(&parameter.value, context, None).with_context(|| {
            format!(
                "could not resolve parameter '{}' in step '{}'",
                parameter.name, step.step_id
            )
        })?;
        let text = scalar_text(&value)?;
        match parameter.location.as_str() {
            "path" => {
                let marker = format!("{{{}}}", parameter.name);
                if !path.contains(&marker) {
                    bail!(
                        "path parameter '{}' is not present in operation path '{}'",
                        parameter.name,
                        path
                    );
                }
                path = path.replace(&marker, &percent_encode_path(&text));
            }
            "query" => query.push((parameter.name.clone(), text)),
            "header" => {
                headers.insert(parameter.name.clone(), text);
            }
            "cookie" => cookies.push(format!("{}={}", parameter.name, text)),
            other => bail!(
                "parameter '{}' uses unsupported location '{}'",
                parameter.name,
                other
            ),
        }
    }
    if path.contains('{') {
        bail!(
            "step '{}' did not supply every path parameter for '{}'",
            step.step_id,
            path
        );
    }
    if !cookies.is_empty() {
        let joined = cookies.join("; ");
        headers
            .entry("Cookie".to_owned())
            .and_modify(|current| {
                current.push_str("; ");
                current.push_str(&joined);
            })
            .or_insert(joined);
    }

    let url = format!(
        "{}/{}",
        base.trim_end_matches('/'),
        path.trim_start_matches('/')
    );
    let method = Method::from_bytes(operation.method.as_bytes()).context("invalid HTTP method")?;
    let mut request = client.request(method, &url).query(&query);
    for (name, value) in &headers {
        request = request.header(name, value);
    }
    let body = if let Some(request_body) = &step.request_body {
        if !request_body
            .content_type
            .to_ascii_lowercase()
            .starts_with("application/json")
        {
            bail!(
                "step '{}' uses unsupported request content type '{}'",
                step.step_id,
                request_body.content_type
            );
        }
        headers.insert("Content-Type".to_owned(), request_body.content_type.clone());
        let value = resolve_value(&request_body.payload, context, None).with_context(|| {
            format!("could not resolve request body in step '{}'", step.step_id)
        })?;
        request = request
            .header("Content-Type", &request_body.content_type)
            .json(&value);
        Some(value)
    } else {
        None
    };

    let built = request
        .build()
        .with_context(|| format!("could not build request for step '{}'", step.step_id))?;
    let final_url = built.url().to_string();
    let response = client
        .execute(built)
        .with_context(|| format!("request failed in step '{}'", step.step_id))?;
    let status = response.status().as_u16();
    let response_headers = response
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            let lower = name.as_str();
            if matches!(
                lower,
                "date" | "server" | "connection" | "keep-alive" | "transfer-encoding"
            ) {
                return None;
            }
            value
                .to_str()
                .ok()
                .map(|value| (name.to_string(), value.to_owned()))
        })
        .collect::<BTreeMap<_, _>>();
    let bytes = response
        .bytes()
        .with_context(|| format!("could not read response body in step '{}'", step.step_id))?;
    let response_body = if bytes.is_empty() {
        None
    } else {
        Some(
            serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned())),
        )
    };
    let response_context = ResponseContext {
        status,
        headers: response_headers.clone(),
        body: response_body.clone(),
    };

    let assertions =
        step.success_criteria
            .iter()
            .map(|criterion| {
                let mut assertion =
                    evaluate_assertion(&criterion.condition, context, &response_context)
                        .with_context(|| {
                            format!(
                                "invalid success criterion in step '{}': {}",
                                step.step_id, criterion.condition
                            )
                        })?;
                assertion.actual = redact_json(assertion.actual, &environment.redact, secrets);
                assertion.expected = redact_json(assertion.expected, &environment.redact, secrets);
                Ok(assertion)
            })
            .collect::<Result<Vec<_>>>()?;
    let outputs = step
        .outputs
        .iter()
        .map(|(name, expression)| {
            let value = resolve_expression(expression, context, Some(&response_context))
                .with_context(|| {
                    format!(
                        "could not resolve output '{}' in step '{}'",
                        name, step.step_id
                    )
                })?;
            Ok((name.clone(), value))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    let result = if assertions.iter().all(|assertion| assertion.passed) {
        RunStatus::Passed
    } else {
        RunStatus::Failed
    };
    context
        .step_outputs
        .insert(step.step_id.clone(), outputs.clone());

    Ok(StepEvidence {
        step_id: step.step_id.clone(),
        operation_id: step.operation_id.clone().or_else(|| {
            operation
                .value
                .get("operationId")
                .and_then(Value::as_str)
                .map(str::to_owned)
        }),
        result,
        request: RequestEvidence {
            method: operation.method.clone(),
            url: redact_string(&final_url, secrets),
            headers: redact_headers(&headers, secrets),
            body: body.map(|value| redact_json(value, &environment.redact, secrets)),
        },
        response: ResponseEvidence {
            status,
            headers: redact_headers(&response_headers, secrets),
            body: response_body.map(|value| redact_json(value, &environment.redact, secrets)),
        },
        assertions,
        outputs: outputs
            .into_iter()
            .map(|(name, value)| (name, redact_json(value, &environment.redact, secrets)))
            .collect(),
    })
}

fn resolve_value(
    value: &Value,
    context: &EvalContext,
    response: Option<&ResponseContext>,
) -> Result<Value> {
    match value {
        Value::String(text) => resolve_string(text, context, response),
        Value::Array(items) => Ok(Value::Array(
            items
                .iter()
                .map(|item| resolve_value(item, context, response))
                .collect::<Result<_>>()?,
        )),
        Value::Object(map) => Ok(Value::Object(
            map.iter()
                .map(|(key, value)| Ok((key.clone(), resolve_value(value, context, response)?)))
                .collect::<Result<Map<_, _>>>()?,
        )),
        other => Ok(other.clone()),
    }
}

fn resolve_string(
    text: &str,
    context: &EvalContext,
    response: Option<&ResponseContext>,
) -> Result<Value> {
    let exact = text
        .strip_prefix('{')
        .and_then(|inner| inner.strip_suffix('}'))
        .unwrap_or(text);
    if exact.starts_with('$') && !exact.contains(' ') {
        return resolve_expression(exact, context, response);
    }
    let mut output = text.to_owned();
    while let Some(start) = output.find("{$") {
        let Some(relative_end) = output[start..].find('}') else {
            bail!("unterminated runtime expression in '{}'", text);
        };
        let end = start + relative_end;
        let expression = &output[start + 1..end];
        let value = resolve_expression(expression, context, response)?;
        output.replace_range(start..=end, &scalar_text(&value)?);
    }
    Ok(Value::String(output))
}

fn resolve_expression(
    expression: &str,
    context: &EvalContext,
    response: Option<&ResponseContext>,
) -> Result<Value> {
    if let Some(name) = expression.strip_prefix("$inputs.") {
        return context
            .inputs
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow!("workflow input '{}' is not defined", name));
    }
    if let Some(name) = expression.strip_prefix("$env.") {
        return context
            .env
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow!("environment value '{}' is not defined", name));
    }
    if let Some(rest) = expression.strip_prefix("$steps.") {
        let Some((step_id, output)) = rest.split_once(".outputs.") else {
            bail!("unsupported step runtime expression '{}'", expression);
        };
        return context
            .step_outputs
            .get(step_id)
            .and_then(|outputs| outputs.get(output))
            .cloned()
            .ok_or_else(|| anyhow!("step output '{}.{}' is not available", step_id, output));
    }
    let response = response.ok_or_else(|| {
        anyhow!(
            "response expression '{}' is unavailable before the request",
            expression
        )
    })?;
    if expression == "$statusCode" {
        return Ok(Value::Number(Number::from(response.status)));
    }
    if let Some(pointer) = expression.strip_prefix("$response.body#") {
        let body = response
            .body
            .as_ref()
            .ok_or_else(|| anyhow!("response has no body"))?;
        return body
            .pointer(pointer)
            .cloned()
            .ok_or_else(|| anyhow!("response pointer '{}' was not found", pointer));
    }
    if let Some(name) = expression.strip_prefix("$response.header.") {
        return response
            .headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| Value::String(value.clone()))
            .ok_or_else(|| anyhow!("response header '{}' was not found", name));
    }
    bail!("unsupported runtime expression '{}'", expression)
}

fn evaluate_assertion(
    condition: &str,
    context: &EvalContext,
    response: &ResponseContext,
) -> Result<AssertionEvidence> {
    let (left, operator, right) = split_condition(condition)?;
    let actual = resolve_expression(left.trim(), context, Some(response))?;
    let expected: Value = serde_yaml::from_str(right.trim())
        .unwrap_or_else(|_| Value::String(right.trim().trim_matches(['\'', '"']).to_owned()));
    let ordering = compare_values(&actual, &expected);
    let passed = match operator {
        "==" => actual == expected,
        "!=" => actual != expected,
        ">" => ordering == Some(std::cmp::Ordering::Greater),
        ">=" => matches!(
            ordering,
            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
        ),
        "<" => ordering == Some(std::cmp::Ordering::Less),
        "<=" => matches!(
            ordering,
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        ),
        _ => unreachable!(),
    };
    Ok(AssertionEvidence {
        condition: condition.to_owned(),
        passed,
        actual,
        expected,
    })
}

fn split_condition(condition: &str) -> Result<(&str, &str, &str)> {
    for operator in [">=", "<=", "!=", "==", ">", "<"] {
        if let Some(index) = condition.find(operator) {
            let (left, rest) = condition.split_at(index);
            let right = &rest[operator.len()..];
            if left.trim().is_empty() || right.trim().is_empty() {
                bail!("condition '{}' is missing an operand", condition);
            }
            return Ok((left, operator, right));
        }
    }
    bail!(
        "unsupported condition '{}'; expected ==, !=, >, >=, <, or <=",
        condition
    )
}

fn compare_values(left: &Value, right: &Value) -> Option<std::cmp::Ordering> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.as_f64()?.partial_cmp(&right.as_f64()?),
        (Value::String(left), Value::String(right)) => Some(left.cmp(right)),
        _ => None,
    }
}

fn scalar_text(value: &Value) -> Result<String> {
    match value {
        Value::String(value) => Ok(value.clone()),
        Value::Number(value) => Ok(value.to_string()),
        Value::Bool(value) => Ok(value.to_string()),
        Value::Null => Ok("null".to_owned()),
        _ => bail!("objects and arrays cannot be used as path, query, header, or template values"),
    }
}

fn percent_encode_path(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn resolves_typed_and_embedded_expressions() {
        let context = EvalContext {
            inputs: BTreeMap::from([("count".into(), json!(3))]),
            env: BTreeMap::from([("zone".into(), json!("north"))]),
            step_outputs: BTreeMap::new(),
        };
        assert_eq!(
            resolve_string("$inputs.count", &context, None).unwrap(),
            json!(3)
        );
        assert_eq!(
            resolve_string("run-{$env.zone}", &context, None).unwrap(),
            json!("run-north")
        );
    }

    #[test]
    fn evaluates_status_and_body_criteria() {
        let response = ResponseContext {
            status: 201,
            headers: BTreeMap::new(),
            body: Some(json!({"ok": true})),
        };
        let context = EvalContext::default();
        assert!(
            evaluate_assertion("$statusCode >= 200", &context, &response)
                .unwrap()
                .passed
        );
        assert!(
            evaluate_assertion("$response.body#/ok == true", &context, &response)
                .unwrap()
                .passed
        );
    }
}
