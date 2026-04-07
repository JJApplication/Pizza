use crate::error::{Result, WasmPluginError};
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;
use wasmtime::{Engine, Linker, Module, Store};

pub struct WasmPlugin {
    name: String,
    module: Module,
    has_request_handler: bool,
    has_response_handler: bool,
}

impl WasmPlugin {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn has_request_handler(&self) -> bool {
        self.has_request_handler
    }
    pub fn has_response_handler(&self) -> bool {
        self.has_response_handler
    }
}

pub struct PluginManager {
    engine: Engine,
    plugins: Arc<RwLock<Vec<WasmPlugin>>>,
    plugin_dir: Option<PathBuf>,
}

impl PluginManager {
    pub fn new(plugin_dir: Option<PathBuf>) -> Result<Self> {
        let mut config = wasmtime::Config::new();
        config.wasm_reference_types(true);

        let engine =
            Engine::new(&config).map_err(|e| WasmPluginError::LoadFailed(e.to_string()))?;

        Ok(Self {
            engine,
            plugins: Arc::new(RwLock::new(Vec::new())),
            plugin_dir,
        })
    }

    pub fn load_plugins(&self) -> Result<()> {
        let plugin_dir = match &self.plugin_dir {
            Some(dir) => dir,
            None => return Ok(()),
        };

        if !plugin_dir.exists() {
            tracing::warn!(path = ?plugin_dir, "Plugin directory does not exist");
            return Ok(());
        }

        let mut plugins = self.plugins.write();

        for entry in
            std::fs::read_dir(plugin_dir).map_err(|e| WasmPluginError::LoadFailed(e.to_string()))?
        {
            let entry = entry.map_err(|e| WasmPluginError::LoadFailed(e.to_string()))?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) == Some("wasm") {
                let plugin = self.load_plugin(&path)?;
                tracing::info!(name = %plugin.name, "Loaded WASM plugin");
                plugins.push(plugin);
            }
        }

        tracing::info!(count = plugins.len(), "WASM plugins loaded");
        Ok(())
    }

    fn load_plugin(&self, path: &PathBuf) -> Result<WasmPlugin> {
        let module = Module::from_file(&self.engine, path)
            .map_err(|e| WasmPluginError::LoadFailed(format!("{}: {}", path.display(), e)))?;

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let has_request_handler = module.get_export("request_handle").is_some();
        let has_response_handler = module.get_export("response_handle").is_some();

        Ok(WasmPlugin {
            name,
            module,
            has_request_handler,
            has_response_handler,
        })
    }

    pub fn execute_request_phase(&self, request_json: &str) -> Result<String> {
        let plugins = self.plugins.read();
        let mut current_request = request_json.to_string();

        for plugin in plugins.iter() {
            if plugin.has_request_handler {
                current_request = self.execute_plugin(plugin, &current_request)?;
            }
        }

        Ok(current_request)
    }

    pub fn execute_response_phase(&self, response_json: &str) -> Result<String> {
        let plugins = self.plugins.read();
        let mut current_response = response_json.to_string();

        for plugin in plugins.iter() {
            if plugin.has_response_handler {
                current_response = self.execute_plugin(plugin, &current_response)?;
            }
        }

        Ok(current_response)
    }

    fn execute_plugin(&self, plugin: &WasmPlugin, input: &str) -> Result<String> {
        let mut store = Store::new(&self.engine, ());
        let linker = Linker::new(&self.engine);

        let instance = linker
            .instantiate(&mut store, &plugin.module)
            .map_err(|e| WasmPluginError::ExecutionError(e.to_string()))?;

        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| WasmPluginError::MemoryError("No memory exported".to_string()))?;

        let handle_fn = if plugin.has_request_handler {
            instance.get_typed_func::<(u32, u32), u32>(&mut store, "request_handle")
        } else {
            instance.get_typed_func::<(u32, u32), u32>(&mut store, "response_handle")
        };

        match handle_fn {
            Ok(func) => {
                let input_bytes = input.as_bytes();
                let input_len = input_bytes.len() as u32;

                let alloc = instance
                    .get_typed_func::<u32, u32>(&mut store, "alloc")
                    .or_else(|_| instance.get_typed_func::<u32, u32>(&mut store, "malloc"));

                if let Ok(alloc_fn) = alloc {
                    let ptr = alloc_fn
                        .call(&mut store, input_len)
                        .map_err(|e| WasmPluginError::MemoryError(e.to_string()))?;

                    memory
                        .write(&mut store, ptr as usize, input_bytes)
                        .map_err(|e| WasmPluginError::MemoryError(e.to_string()))?;

                    let result_ptr = func
                        .call(&mut store, (ptr, input_len))
                        .map_err(|e| WasmPluginError::ExecutionError(e.to_string()))?;

                    let result_len = 1024u32;
                    let mut result_buf = vec![0u8; result_len as usize];
                    memory
                        .read(&mut store, result_ptr as usize, &mut result_buf)
                        .map_err(|e| WasmPluginError::MemoryError(e.to_string()))?;

                    let result = String::from_utf8_lossy(&result_buf)
                        .trim_end_matches('\0')
                        .to_string();

                    Ok(result)
                } else {
                    Err(
                        WasmPluginError::MemoryError("No alloc/malloc function found".to_string())
                            .into(),
                    )
                }
            }
            Err(_) => Ok(input.to_string()),
        }
    }

    pub fn plugin_count(&self) -> usize {
        self.plugins.read().len()
    }

    pub fn plugin_names(&self) -> Vec<String> {
        self.plugins.read().iter().map(|p| p.name.clone()).collect()
    }
}
