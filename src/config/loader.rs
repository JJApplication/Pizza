use crate::config::app_config::AppConfig;
use crate::error::{ConfigError, Result};
use std::fs;
use std::path::Path;

pub fn load_app_config(path: &str) -> Result<AppConfig> {
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::LoadFailed(format!("{}: {}", path, e)))?;

    let config = parse_config(path, &content)?;
    Ok(config)
}

pub fn parse_config(path: &str, content: &str) -> Result<AppConfig> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("json");

    match ext {
        "json" => parse_json(content),
        "toml" => parse_toml(content),
        _ => parse_json(content),
    }
}

fn parse_json(content: &str) -> Result<AppConfig> {
    serde_json::from_str(content)
        .map_err(|e| ConfigError::ParseFailed(e.to_string()))
        .map_err(Into::into)
}

fn parse_toml(content: &str) -> Result<AppConfig> {
    toml::from_str(content)
        .map_err(|e| ConfigError::ParseFailed(e.to_string()))
        .map_err(Into::into)
}

pub fn load_json_file(path: &str) -> Result<serde_json::Value> {
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::LoadFailed(format!("{}: {}", path, e)))?;
    serde_json::from_str(&content)
        .map_err(|e| ConfigError::ParseFailed(e.to_string()))
        .map_err(Into::into)
}

pub fn load_toml_file(path: &str) -> Result<toml::Value> {
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::LoadFailed(format!("{}: {}", path, e)))?;
    toml::from_str(&content)
        .map_err(|e| ConfigError::ParseFailed(e.to_string()))
        .map_err(Into::into)
}
