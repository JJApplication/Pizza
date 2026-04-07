use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DomainMapConfig {
    #[serde(flatten)]
    pub domains: HashMap<String, DomainEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DomainEntry {
    #[serde(default)]
    pub frontend: String,
    #[serde(default)]
    pub backend: String,
}
