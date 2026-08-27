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
            Ok(OpenApiSource {
                name: source.name.clone(),
                path,
                document,
            })
        })
        .collect()
}
