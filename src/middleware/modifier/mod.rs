use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::Full;

pub trait Modifier: Send + Sync {
    fn name(&self) -> &str;
    fn enabled(&self) -> bool;
    fn modify(
        &self,
        req: &Request<Full<Bytes>>,
        resp: &mut Response<Full<Bytes>>,
    ) -> Result<(), String>;
}

pub struct ModifierManager {
    modifiers: Vec<Box<dyn Modifier>>,
}

impl ModifierManager {
    pub fn new() -> Self {
        Self {
            modifiers: Vec::new(),
        }
    }

    pub fn register(&mut self, modifier: Box<dyn Modifier>) {
        self.modifiers.push(modifier);
    }

    pub fn execute(
        &self,
        req: &Request<Full<Bytes>>,
        resp: &mut Response<Full<Bytes>>,
    ) -> Result<(), String> {
        for modifier in &self.modifiers {
            if modifier.enabled() {
                modifier.modify(req, resp)?;
            }
        }
        Ok(())
    }

    pub fn enabled_names(&self) -> Vec<&str> {
        self.modifiers
            .iter()
            .filter(|m| m.enabled())
            .map(|m| m.name())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.modifiers.len()
    }
}

impl Default for ModifierManager {
    fn default() -> Self {
        Self::new()
    }
}
