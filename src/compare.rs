use crate::model::{Change, Comparison, ComparisonSummary, ProofBundle, StepEvidence};
use crate::redact::{REDACTED, sensitive_header};
use crate::report::render_comparison;
use anyhow::{Context, Result};
use serde_json::{Value, to_value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// Compare the review-relevant fields of two already-redacted proof bundles.
pub fn compare_proofs(baseline: &ProofBundle, current: &ProofBundle) -> Comparison {
    let before: BTreeMap<&str, &StepEvidence> = baseline
        .steps
        .iter()
        .map(|step| (step.step_id.as_str(), step))
        .collect();
    let after: BTreeMap<&str, &StepEvidence> = current
        .steps
        .iter()
        .map(|step| (step.step_id.as_str(), step))
        .collect();
    let ids: BTreeSet<&str> = before.keys().chain(after.keys()).copied().collect();
    let mut changes = Vec::new();
    let mut added = 0;
    let mut removed = 0;
    let mut modified_steps = BTreeSet::new();

    for id in ids {
        match (before.get(id), after.get(id)) {
            (None, Some(step)) => {
                added += 1;
                changes.push(Change {
                    step_id: id.to_owned(),
                    field: "step".to_owned(),
                    before: None,
                    after: to_value(step).ok().map(scrub_sensitive_fields),
                });
            }
            (Some(step), None) => {
                removed += 1;
                changes.push(Change {
                    step_id: id.to_owned(),
                    field: "step".to_owned(),
                    before: to_value(step).ok().map(scrub_sensitive_fields),
                    after: None,
                });
            }
            (Some(left), Some(right)) => {
                compare_field(
                    id,
                    "request",
                    &left.request,
                    &right.request,
                    &mut changes,
                    &mut modified_steps,
                );
                compare_field(
                    id,
                    "response.status",
                    &left.response.status,
                    &right.response.status,
                    &mut changes,
                    &mut modified_steps,
                );
                compare_field(
                    id,
                    "response.headers",
                    &left.response.headers,
                    &right.response.headers,
                    &mut changes,
                    &mut modified_steps,
                );
                compare_field(
                    id,
                    "response.body",
                    &left.response.body,
                    &right.response.body,
                    &mut changes,
                    &mut modified_steps,
                );
                compare_field(
                    id,
                    "assertions",
                    &left.assertions,
                    &right.assertions,
                    &mut changes,
                    &mut modified_steps,
                );
                compare_field(
                    id,
                    "outputs",
                    &left.outputs,
                    &right.outputs,
                    &mut changes,
                    &mut modified_steps,
                );
            }
            (None, None) => unreachable!(),
        }
    }

    Comparison {
        schema_version: "arazzo-proof-comparison/v1".to_owned(),
        baseline_workflow: baseline.workflow_id.clone(),
        current_workflow: current.workflow_id.clone(),
        changed: !changes.is_empty(),
        summary: ComparisonSummary {
            added,
            removed,
            modified: modified_steps.len(),
        },
        changes,
    }
}

/// Write `comparison.json` and a self-contained `comparison.html`.
pub fn write_comparison(comparison: &Comparison, output: &Path) -> Result<()> {
    fs::create_dir_all(output).with_context(|| format!("could not create {}", output.display()))?;
    let json =
        serde_json::to_string_pretty(comparison).context("could not serialize comparison")?;
    fs::write(output.join("comparison.json"), format!("{json}\n"))
        .with_context(|| format!("could not write {}/comparison.json", output.display()))?;
    fs::write(
        output.join("comparison.html"),
        render_comparison(comparison),
    )
    .with_context(|| format!("could not write {}/comparison.html", output.display()))?;
    Ok(())
}

fn compare_field<T: serde::Serialize + PartialEq>(
    step_id: &str,
    field: &str,
    before: &T,
    after: &T,
    changes: &mut Vec<Change>,
    modified_steps: &mut BTreeSet<String>,
) {
    if before != after {
        modified_steps.insert(step_id.to_owned());
        changes.push(Change {
            step_id: step_id.to_owned(),
            field: field.to_owned(),
            before: to_value(before)
                .ok()
                .map(scrub_sensitive_fields)
                .or(Some(Value::Null)),
            after: to_value(after)
                .ok()
                .map(scrub_sensitive_fields)
                .or(Some(Value::Null)),
        });
    }
}

fn scrub_sensitive_fields(mut value: Value) -> Value {
    match &mut value {
        Value::Object(map) => {
            for (key, child) in map {
                if sensitive_header(key) {
                    *child = Value::String(REDACTED.to_owned());
                } else {
                    *child = scrub_sensitive_fields(std::mem::take(child));
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                *item = scrub_sensitive_fields(std::mem::take(item));
            }
        }
        _ => {}
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    use std::collections::BTreeMap;

    fn proof(status: u16, assertion_passed: bool) -> ProofBundle {
        ProofBundle {
            schema_version: "arazzo-proof/v1".into(),
            arazzo_version: "1.0.1".into(),
            workflow_id: "checkout".into(),
            environment: "test".into(),
            source_file: "flow.yaml".into(),
            result: if assertion_passed {
                RunStatus::Passed
            } else {
                RunStatus::Failed
            },
            summary: ProofSummary {
                steps: 1,
                assertions: 1,
                passed: usize::from(assertion_passed),
                failed: usize::from(!assertion_passed),
            },
            steps: vec![StepEvidence {
                step_id: "pay".into(),
                operation_id: Some("pay".into()),
                result: RunStatus::Passed,
                request: RequestEvidence {
                    method: "POST".into(),
                    url: "https://example.test/pay".into(),
                    headers: BTreeMap::new(),
                    body: None,
                },
                response: ResponseEvidence {
                    status,
                    headers: BTreeMap::new(),
                    body: None,
                },
                assertions: vec![AssertionEvidence {
                    condition: "$statusCode == 200".into(),
                    passed: assertion_passed,
                    actual: status.into(),
                    expected: 200.into(),
                }],
                outputs: BTreeMap::new(),
            }],
        }
    }

    #[test]
    fn exposes_changed_status_and_assertion() {
        let comparison = compare_proofs(&proof(200, true), &proof(500, false));
        assert!(comparison.changed);
        assert!(
            comparison
                .changes
                .iter()
                .any(|change| change.field == "response.status")
        );
        assert!(
            comparison
                .changes
                .iter()
                .any(|change| change.field == "assertions")
        );
    }

    #[test]
    fn never_copies_authorization_values_into_comparisons() {
        let baseline = proof(200, true);
        let mut current = baseline.clone();
        current.steps[0]
            .request
            .headers
            .insert("Authorization".into(), "Bearer do-not-write".into());
        let serialized = serde_json::to_string(&compare_proofs(&baseline, &current)).unwrap();
        assert!(!serialized.contains("do-not-write"));
        assert!(serialized.contains(REDACTED));
    }
}
