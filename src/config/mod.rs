pub mod app_config;
pub mod backend_config;
pub mod frontend_config;
pub mod loader;
pub mod merge;
pub mod server_config;

pub use app_config::*;
pub use backend_config::*;
pub use frontend_config::*;
pub use loader::*;
pub use merge::*;
pub use server_config::*;

use parking_lot::RwLock;
use std::sync::OnceLock;

static GLOBAL_CONFIG: OnceLock<RwLock<Option<merge::MergedConfig>>> = OnceLock::new();

fn global_config() -> &'static RwLock<Option<merge::MergedConfig>> {
    GLOBAL_CONFIG.get_or_init(|| RwLock::new(None))
}

pub fn set_config(config: merge::MergedConfig) {
    *global_config().write() = Some(config);
}

pub fn get_config() -> Option<merge::MergedConfig> {
    global_config().read().as_ref().cloned()
}

pub fn with_config<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&merge::MergedConfig) -> R,
{
    let guard = global_config().read();
    guard.as_ref().map(|c| f(c))
}
