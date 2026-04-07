use crate::config::app_config::{CustomServiceConfig, Upstream};
use bytes::Bytes;
use http_body_util::Full;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone)]
pub struct ServiceMapping {
    pub path_prefix: String,
    pub backend: String,
    pub port: Option<u16>,
    pub scheme: Option<String>,
    pub rewrite: Option<String>,
    pub strip_prefix: bool,
}

#[derive(Debug, Clone)]
pub struct ResolvedTarget {
    pub host: String,
    pub port: u16,
    pub scheme: String,
    pub rewrite: Option<String>,
    pub strip_prefix: bool,
    pub is_custom_service: bool,
    pub target_service: Option<String>,
}

pub struct Resolver {
    domain_map: HashMap<String, Vec<ServiceMapping>>,
    default_backend: Option<String>,
    custom_services: Vec<CustomServiceConfig>,
    custom_service_enabled: bool,
    rr_counter: AtomicUsize,
}

impl Resolver {
    pub fn new(
        domain_map: HashMap<String, Vec<ServiceMapping>>,
        default_backend: Option<String>,
        custom_services: Vec<CustomServiceConfig>,
        custom_service_enabled: bool,
    ) -> Self {
        Self {
            domain_map,
            default_backend,
            custom_services,
            custom_service_enabled,
            rr_counter: AtomicUsize::new(0),
        }
    }

    pub fn resolve(&self, host: &str, path: &str) -> Option<ResolvedTarget> {
        if let Some(mappings) = self.domain_map.get(host) {
            for mapping in mappings {
                if path.starts_with(&mapping.path_prefix) {
                    return Some(ResolvedTarget {
                        host: mapping.backend.clone(),
                        port: mapping.port.unwrap_or(80),
                        scheme: mapping.scheme.clone().unwrap_or_else(|| "http".to_string()),
                        rewrite: mapping.rewrite.clone(),
                        strip_prefix: mapping.strip_prefix,
                        is_custom_service: false,
                        target_service: None,
                    });
                }
            }
        }

        if let Some(backend) = &self.default_backend {
            return Some(ResolvedTarget {
                host: backend.clone(),
                port: 80,
                scheme: "http".to_string(),
                rewrite: None,
                strip_prefix: false,
                is_custom_service: false,
                target_service: None,
            });
        }

        None
    }

    pub fn resolve_custom_service(&self, host: &str, path: &str) -> Option<ResolvedTarget> {
        if !self.custom_service_enabled {
            return None;
        }

        for service in &self.custom_services {
            if host == service.domain {
                for api in &service.apis {
                    if api.api.is_empty() {
                        continue;
                    }
                    if path.starts_with(&api.api) {
                        let target_path = if api.use_rewrite && !api.rewrite.is_empty() {
                            format!("{}{}", api.rewrite, &path[api.api.len()..])
                        } else {
                            path.to_string()
                        };

                        if !api.target_service.is_empty() {
                            return Some(ResolvedTarget {
                                host: "127.0.0.1".to_string(),
                                port: 0,
                                scheme: "http".to_string(),
                                rewrite: Some(target_path),
                                strip_prefix: false,
                                is_custom_service: true,
                                target_service: Some(api.target_service.clone()),
                            });
                        }

                        let target_host = if !api.target_host.is_empty() {
                            api.target_host.clone()
                        } else if !api.target.is_empty() {
                            if let Some(parsed) = url::Url::parse(&api.target).ok() {
                                parsed.host_str().unwrap_or("").to_string()
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        };

                        let target_port = if api.target_port > 0 {
                            api.target_port as u16
                        } else if !api.target.is_empty() {
                            if let Ok(parsed) = url::Url::parse(&api.target) {
                                parsed.port_or_known_default().unwrap_or(80) as u16
                            } else {
                                80
                            }
                        } else {
                            80
                        };

                        return Some(ResolvedTarget {
                            host: target_host,
                            port: target_port,
                            scheme: "http".to_string(),
                            rewrite: Some(target_path),
                            strip_prefix: false,
                            is_custom_service: true,
                            target_service: None,
                        });
                    }
                }

                if !service.upstream.is_empty() {
                    let picked = self.pick_round_robin(&service.upstream);
                    return Some(ResolvedTarget {
                        host: picked.host.clone(),
                        port: picked.port as u16,
                        scheme: "http".to_string(),
                        rewrite: None,
                        strip_prefix: false,
                        is_custom_service: true,
                        target_service: None,
                    });
                }
            }
        }

        None
    }

    fn pick_round_robin<'a>(&self, addrs: &'a [Upstream]) -> &'a Upstream {
        if addrs.is_empty() {
            return &addrs[0];
        }
        let idx = self.rr_counter.fetch_add(1, Ordering::Relaxed) % addrs.len();
        &addrs[idx]
    }

    pub fn update_domain_map(&mut self, domain_map: HashMap<String, Vec<ServiceMapping>>) {
        self.domain_map = domain_map;
    }

    pub fn get_domain_map(&self) -> &HashMap<String, Vec<ServiceMapping>> {
        &self.domain_map
    }

    pub fn is_custom_service_enabled(&self) -> bool {
        self.custom_service_enabled
    }
}
