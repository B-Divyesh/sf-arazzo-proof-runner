use serde_json::Value;
use std::collections::BTreeMap;

pub(crate) const REDACTED: &str = "[REDACTED]";

pub(crate) fn sensitive_header(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    matches!(
        name.as_str(),
        "authorization" | "proxy-authorization" | "cookie" | "set-cookie" | "x-api-key"
    ) || name.contains("token")
        || name.contains("secret")
        || name.contains("api-key")
}

pub(crate) fn secret_values(
    headers: &BTreeMap<String, String>,
    explicit: &[String],
) -> Vec<String> {
    let mut values: Vec<String> = explicit.iter().filter(|v| !v.is_empty()).cloned().collect();
    for (name, value) in headers {
        if sensitive_header(name) {
            values.push(value.clone());
            if let Some((_, token)) = value.split_once(' ')
                && token.len() >= 4
            {
                values.push(token.to_owned());
            }
        }
    }
    values.sort_by_key(|b| std::cmp::Reverse(b.len()));
    values.dedup();
    values
}

pub(crate) fn redact_headers(
    headers: &BTreeMap<String, String>,
    secrets: &[String],
) -> BTreeMap<String, String> {
    headers
        .iter()
        .map(|(name, value)| {
            let value = if sensitive_header(name) {
                REDACTED.to_owned()
            } else {
                redact_string(value, secrets)
            };
            (name.clone(), value)
        })
        .collect()
}

pub(crate) fn redact_string(input: &str, secrets: &[String]) -> String {
    secrets
        .iter()
        .filter(|s| !s.is_empty())
        .fold(input.to_owned(), |text, secret| {
            text.replace(secret, REDACTED)
        })
}

pub(crate) fn redact_json(mut value: Value, pointers: &[String], secrets: &[String]) -> Value {
    for pointer in pointers {
        if pointer.is_empty() {
            value = Value::String(REDACTED.to_owned());
        } else if let Some(target) = value.pointer_mut(pointer) {
            *target = Value::String(REDACTED.to_owned());
        }
    }
    redact_json_strings(&mut value, secrets);
    value
}

fn redact_json_strings(value: &mut Value, secrets: &[String]) {
    match value {
        Value::String(text) => *text = redact_string(text, secrets),
        Value::Array(items) => items
            .iter_mut()
            .for_each(|item| redact_json_strings(item, secrets)),
        Value::Object(map) => map
            .values_mut()
            .for_each(|item| redact_json_strings(item, secrets)),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn redacts_headers_pointers_and_nested_secret_values() {
        let mut headers = BTreeMap::new();
        headers.insert("Authorization".to_owned(), "Bearer abc123".to_owned());
        headers.insert("X-Trace".to_owned(), "trace-abc123".to_owned());
        let secrets = secret_values(&headers, &[]);
        assert_eq!(
            redact_headers(&headers, &secrets)["Authorization"],
            REDACTED
        );
        assert_eq!(
            redact_headers(&headers, &secrets)["X-Trace"],
            "trace-[REDACTED]"
        );
        let body = redact_json(
            json!({"owner":{"email":"a@b.test"},"token":"abc123"}),
            &["/owner/email".to_owned()],
            &secrets,
        );
        assert_eq!(body["owner"]["email"], REDACTED);
        assert_eq!(body["token"], REDACTED);
    }
}
