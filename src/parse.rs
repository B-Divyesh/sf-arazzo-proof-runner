use anyhow::{Context, Result, bail};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArazzoDocument {
    pub arazzo: String,
    #[serde(default)]
    pub source_descriptions: Vec<SourceDescription>,
    pub workflows: Vec<Workflow>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct SourceDescription {
    pub name: String,
    pub url: String,
    #[serde(rename = "type", default)]
    pub kind: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Workflow {
    pub workflow_id: String,
    #[serde(default)]
    pub inputs: Option<Value>,
    pub steps: Vec<Step>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Step {
    pub step_id: String,
    #[serde(default)]
    pub operation_id: Option<String>,
    #[serde(default)]
    pub operation_path: Option<String>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    #[serde(default)]
    pub request_body: Option<RequestBody>,
    #[serde(default)]
    pub success_criteria: Vec<Criterion>,
    #[serde(default)]
    pub outputs: BTreeMap<String, String>,
    #[serde(default)]
    pub on_success: Vec<Value>,
    #[serde(default)]
    pub on_failure: Vec<Value>,
    #[serde(default)]
    pub retry_after: Option<Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,
    pub value: Value,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequestBody {
    #[serde(default = "default_content_type")]
    pub content_type: String,
    #[serde(default)]
    pub payload: Value,
}

fn default_content_type() -> String {
    "application/json".to_owned()
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Criterion {
    pub condition: String,
    #[serde(default)]
    pub context: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct OpenApiSource {
    pub name: String,
    pub path: PathBuf,
    pub document: Value,
}

pub(crate) fn read_yaml_or_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let text =
        fs::read_to_string(path).with_context(|| format!("could not read {}", path.display()))?;
    if path.extension().and_then(|x| x.to_str()) == Some("json") {
        serde_json::from_str(&text).with_context(|| format!("invalid JSON in {}", path.display()))
    } else {
        serde_yaml::from_str(&text).with_context(|| format!("invalid YAML in {}", path.display()))
    }
}

pub(crate) fn load_sources(doc: &ArazzoDocument, arazzo_path: &Path) -> Result<Vec<OpenApiSource>> {
    if doc.source_descriptions.is_empty() {
        bail!("sourceDescriptions must contain at least one local OpenAPI document");
    }
    let root = arazzo_path.parent().unwrap_or_else(|| Path::new("."));
    doc.source_descriptions
        .iter()
        .map(|source| {
            if source.url.starts_with("http://") || source.url.starts_with("https://") {
                bail!(
                    "remote sourceDescription '{}' is unsupported; use a local OpenAPI file",
                    source.name
                );
            }
            if let Some(kind) = &source.kind
                && !kind.eq_ignore_ascii_case("openapi")
            {
                bail!(
                    "sourceDescription '{}' has unsupported type '{}'",
                    source.name,
                    kind
                );
            }
            let path = root.join(&source.url);
            let document: Value = read_yaml_or_json(&path)?;
            validate_openapi_source(source, &document)?;
            Ok(OpenApiSource {
                name: source.name.clone(),
                path,
                document,
            })
        })
        .collect()
}

fn validate_openapi_source(source: &SourceDescription, document: &Value) -> Result<()> {
    let version = document
        .get("openapi")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "sourceDescription '{}' is not an OpenAPI document",
                source.name
            )
        })?;
    if !(version.starts_with("3.0") || version.starts_with("3.1")) {
        bail!(
            "sourceDescription '{}' uses unsupported OpenAPI version '{}'",
            source.name,
            version
        );
    }
    if document.get("webhooks").is_some() {
        bail!(
            "sourceDescription '{}' uses webhooks, which are unsupported",
            source.name
        );
    }
    if has_key(document, "callbacks") {
        bail!(
            "sourceDescription '{}' uses callbacks, which are unsupported",
            source.name
        );
    }
    if has_external_ref(document) {
        bail!(
            "sourceDescription '{}' uses an external $ref, which is unsupported",
            source.name
        );
    }
    if has_oauth_scheme(document) {
        bail!(
            "sourceDescription '{}' uses an OAuth flow, which is unsupported",
            source.name
        );
    }
    Ok(())
}

fn has_key(value: &Value, wanted: &str) -> bool {
    match value {
        Value::Object(map) => map
            .iter()
            .any(|(key, child)| key == wanted || has_key(child, wanted)),
        Value::Array(items) => items.iter().any(|child| has_key(child, wanted)),
        _ => false,
    }
}

fn has_external_ref(value: &Value) -> bool {
    match value {
        Value::Object(map) => map.iter().any(|(key, child)| {
            (key == "$ref"
                && child
                    .as_str()
                    .is_some_and(|reference| !reference.starts_with('#')))
                || has_external_ref(child)
        }),
        Value::Array(items) => items.iter().any(has_external_ref),
        _ => false,
    }
}

fn has_oauth_scheme(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            map.get("type").and_then(Value::as_str) == Some("oauth2")
                || map.values().any(has_oauth_scheme)
        }
        Value::Array(items) => items.iter().any(has_oauth_scheme),
        _ => false,
    }
}
