pub trait DefaultIfEmpty {
    fn default_if_empty(self, default: Self) -> Self;
}

impl DefaultIfEmpty for String {
    fn default_if_empty(self, default: Self) -> Self {
        if self.is_empty() {
            default
        } else {
            self
        }
    }
}

impl DefaultIfEmpty for Option<String> {
    fn default_if_empty(self, default: Self) -> Self {
        match self {
            Some(s) if !s.is_empty() => Some(s),
            _ => default,
        }
    }
}

pub fn default_true() -> bool {
    true
}
pub fn default_false() -> bool {
    false
}
pub fn default_port() -> u16 {
    80
}
pub fn default_tls_port() -> u16 {
    443
}
pub fn default_max_connections() -> usize {
    1000
}
pub fn default_timeout_secs() -> u64 {
    30
}
pub fn default_buffer_size() -> usize {
    32 * 1024
}
