#![forbid(unsafe_code)]

use anyhow::Result;
use serde::Deserialize;
use serde_yaml::Value;

pub fn convert(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }

    let mut lines = Vec::new();
    for document in serde_yaml::Deserializer::from_str(input) {
        let value = Value::deserialize(document)?;
        lines.push(serde_json::to_string(&value)?);
    }

    if lines.is_empty() {
        return Ok(String::new());
    }

    let mut out = lines.join("\n");
    out.push('\n');
    Ok(out)
}
