use std::{collections::HashMap, path::Path, str::FromStr, sync::Arc};

use crate::core::vm::{Environment, Module, WasmModuleLoader};
use anyhow::anyhow;
use serde::Deserialize;

const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0:5555";

#[derive(Deserialize, Debug, Clone)]
struct ModuleTomlConfigModuleItem {
    bin: String,
    prefix: Option<String>,
    env: Option<HashMap<String, String>>,
    log_level: Option<String>,
    log_path: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct ModuleTomlConfigGlobal {
    bind_address: Option<String>,
    log_level: Option<String>,
    log_path: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct ModuleTomlConfig {
    global: ModuleTomlConfigGlobal,
    modules: HashMap<String, ModuleTomlConfigModuleItem>,
}

impl ModuleTomlConfig {
    async fn from_config_path(config_path: &str) -> anyhow::Result<Self> {
        let modules_config_str = tokio::fs::read_to_string(config_path).await?;
        let modules_config: ModuleTomlConfig = toml::from_str(&modules_config_str).unwrap();
        Ok(modules_config)
    }
}

impl ModuleTomlConfigModuleItem {
    fn validate() -> anyhow::Result<()> {
        Ok(())
    }
}

// TODO for production
struct ModuleLoaderForProduction {}

struct ModuleLoaderForDevelopment {
    config: Arc<ModuleTomlConfig>,
    config_path: String,
}

#[async_trait::async_trait]
impl WasmModuleLoader for ModuleLoaderForDevelopment {
    async fn fetch_module(
        &self,
        engine: Arc<wasmtime::Engine>,
        module_prefix: String,
    ) -> anyhow::Result<Module> {
        let (module_name, module_config) = self
            .config
            .modules
            .clone()
            .into_iter()
            .find(|(module_name, module_config)| {
                module_config.prefix.as_ref().unwrap_or(module_name) == &module_prefix
            })
            .unwrap();

        let module_bin_path = Path::new(&self.config_path)
            .parent()
            .ok_or(anyhow!("not found parent directory"))?
            .join(Path::new(&module_config.bin));

        let binary = tokio::fs::read(module_bin_path).await?;

        let internal_module = wasmtime::Module::from_binary(&engine, &binary)?;

        let envs: Vec<(String, String)> = module_config
            .env
            .unwrap_or(HashMap::new())
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        Ok(Module {
            module_name,
            envs,
            version_digest: "not available in local".to_string(),
            module: internal_module,
        })
    }
}

pub async fn run(run_args: super::RunArgs) -> anyhow::Result<()> {
    use crate::app::{http_entry::make_serve, infra::AppState};

    let config = Arc::new(ModuleTomlConfig::from_config_path(&run_args.config).await?);

    let engine = Environment::new_engine()?;
    let module_fetch = Box::new(ModuleLoaderForDevelopment {
        config: config.clone(),
        config_path: run_args.config.clone(),
    });
    let environment = Environment::new(module_fetch, engine).await?;
    let app_state = AppState {
        environment: environment,
    };

    let bind_address = config
        .global
        .bind_address
        .clone()
        .unwrap_or(DEFAULT_BIND_ADDRESS.to_owned());

    info!("ServiceStart|Listen={}", bind_address);
    make_serve(&bind_address, app_state).await?;
    Ok(())
}
