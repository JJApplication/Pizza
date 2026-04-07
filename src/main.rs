use clap::{Parser, Subcommand};
use pizza::config::{load_app_config, merge_configs};
use pizza::app::PizzaApp;
use pizza::app::lifecycle::LifecycleManager;
use pizza::error::Result;
use pizza::constants::{APP_NAME, VERSION, DESCRIPTION, ENV_CONFIG_PATH};
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[command(name = APP_NAME, version = VERSION, about = DESCRIPTION, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, help = "Path to config file")]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the gateway server
    Run,
    /// Generate a default config file
    Generate {
        #[arg(short, long, default_value = "config.json")]
        output: String,
    },
    /// Validate a config file without starting
    Test {
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Reload the running instance
    Reload,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logging();

    match cli.command.unwrap_or(Commands::Run) {
        Commands::Run => {
            print_banner();
            let config_path = cli.config
                .or_else(|| std::env::var(ENV_CONFIG_PATH).ok())
                .unwrap_or_else(|| "config/config.json".to_string());

            run_gateway(&config_path).await?;
        }
        Commands::Generate { output } => {
            generate_config(&output)?;
        }
        Commands::Test { config } => {
            let config_path = config
                .or_else(|| cli.config)
                .or_else(|| std::env::var(ENV_CONFIG_PATH).ok())
                .unwrap_or_else(|| "config/config.json".to_string());

            test_config(&config_path)?;
        }
        Commands::Reload => {
            reload_instance().await?;
        }
    }

    Ok(())
}

async fn run_gateway(config_path: &str) -> Result<()> {
    tracing::info!(config = %config_path, "Loading configuration");

    let app_config = load_app_config(config_path)?;
    let backend_file = app_config.pxy_backend_file.clone();
    let frontend_file = app_config.pxy_frontend_file.clone();
    let domain_map_file = app_config.domain_map.clone();
    let merged = merge_configs(
        app_config,
        backend_file.as_deref(),
        frontend_file.as_deref(),
        domain_map_file.as_deref(),
    )?;

    let app = PizzaApp::new(merged).await?;
    app.initialize().await?;
    app.print_status();

    let lifecycle = LifecycleManager::new();

    tokio::select! {
        result = app.run() => {
            result?;
        }
        result = lifecycle.wait_for_shutdown() => {
            result?;
        }
    }

    Ok(())
}

fn generate_config(output: &str) -> Result<()> {
    let default_config = r#"{
    "core_proxy": {
        "mode": "http",
        "net_io": "http",
        "buffer_size": 32768,
        "max_connections": 1000,
        "timeout_secs": 30
    },
    "servers": [
        {
            "name": "default",
            "addr": "0.0.0.0:8080",
            "tls": false,
            "domains": []
        }
    ],
    "middleware": {
        "gzip": { "enabled": true, "min_length": 1024, "level": 4 },
        "cors": { "enabled": false, "allow_origins": [], "allow_methods": [], "allow_headers": [] },
        "trace_id": true,
        "secure_headers": true,
        "no_cache": false,
        "custom_headers": [],
        "sanitizer": { "enabled": false, "remove_headers": [] }
    },
    "features": {
        "flow_control": { "enabled": false, "algorithm": "fixed_window", "rules": [] },
        "websocket": true,
        "proxy_cache": false,
        "http3": false,
        "circuit_breaker": { "enabled": false, "bucket_size": 5, "reset_secs": 60 }
    },
    "grpc": { "enabled": false, "addr": "127.0.0.1:50051" },
    "api_server": { "enabled": false, "addr": "127.0.0.1:8081" },
    "database": {
        "mongo": { "uri": "", "db_name": "" }
    },
    "security": {
        "ip_allow_list": [],
        "ip_deny_list": [],
        "rate_limit": { "enabled": false, "requests_per_second": 100 }
    },
    "stat": { "enabled": false, "persist_interval_secs": 60 },
    "plugin": { "enabled": false },
    "pre_auth": { "enabled": false, "auth_type": "" },
    "notify": { "enabled": false },
    "log": { "level": "info", "json_format": false },
    "pprof": { "enabled": false, "addr": "127.0.0.1:6060" }
}"#;

    fs::write(output, default_config)?;
    println!("Default config generated: {}", output);
    Ok(())
}

fn test_config(config_path: &str) -> Result<()> {
    println!("Testing config: {}", config_path);

    match load_app_config(config_path) {
        Ok(config) => {
            println!("Config loaded successfully");
            println!("  Proxy mode: {}", config.proxy.proxy_mode);
            println!("  Net I/O: {}", config.proxy.net_io);
            println!("  Servers: {}", config.servers.len());
            println!("  Middleware - Gzip: {}, CORS: {}, Trace: {}, Secure: {}",
                config.middleware.gzip.enabled,
                config.middleware.cors.enabled,
                config.middleware.trace.enabled,
                config.middleware.secure_header,
            );
            println!("  Flow control: {}", config.features.flow_control.enabled);
            println!("  WASM plugins: {}", config.plugin.enabled);
            println!("Config is valid!");
            Ok(())
        }
        Err(e) => {
            println!("Config validation failed: {}", e);
            Err(e)
        }
    }
}

async fn reload_instance() -> Result<()> {
    println!("Sending reload signal...");
    let pid_path = PathBuf::from("pizza.pid");
    if pid_path.exists() {
        let pid_str = fs::read_to_string(&pid_path)?;
        let pid: u32 = pid_str.trim().parse().map_err(|e| pizza::error::PizzaError::Other(anyhow::anyhow!("Invalid PID: {}", e)))?;
        println!("Found PID: {}", pid);
    } else {
        println!("No PID file found");
    }
    Ok(())
}

fn init_logging() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,pizza=debug".into());

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_thread_ids(false)
        .init();
}

fn print_banner() {
    println!(r#"
$$$$$$$\  $$$$$$\ $$$$$$$$\ $$$$$$$$\  $$$$$$\
$$  __$$\ \_$$  _|\____$$  |\____$$  |$$  __$$\
$$ |  $$ |  $$ |      $$  /     $$  / $$ /  $$ |
$$$$$$$  |  $$ |     $$  /     $$  /  $$$$$$$$ |
$$  ____/   $$ |    $$  /     $$  /   $$  __$$ |
$$ |        $$ |   $$  /     $$  /    $$ |  $$ |
$$ |      $$$$$$\ $$$$$$$$\ $$$$$$$$\ $$ |  $$ |
\__|      \______|\________|\________|\__|  \__|


                                                
"#);
    println!("  {} v{}", APP_NAME, VERSION);
    println!("  {}", DESCRIPTION);
    println!();
}
