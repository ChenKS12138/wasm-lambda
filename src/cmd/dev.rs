use std::sync::Arc;

use super::DevArgs;
use crate::{app::infra::AppState, core};

struct ModuleFetchFromLocal {
    map: std::collections::HashMap<String, String>,
}

#[async_trait::async_trait]
impl core::vm::FetchModule for ModuleFetchFromLocal {
    async fn fetch_module(
        &self,
        engine: Arc<wasmtime::Engine>,
        module_name: String,
        _version_alias: String,
    ) -> anyhow::Result<core::vm::Module> {
        let engine = engine.clone();
        match self.map.get(&module_name) {
            None => Err(anyhow::anyhow!("module not found")),
            Some(file_path) => {
                let internal_module = wasmtime::Module::from_file(&engine, file_path);
                Ok(core::vm::Module {
                    module_name: module_name.clone(),
                    envs: vec![],
                    version_digest: "not available in local".to_string(),
                    module: internal_module?,
                    version_alias: "".to_string(),
                })
            }
        }
    }
}

pub async fn dev(args: DevArgs) -> anyhow::Result<()> {
    let module_fetch = Box::new(ModuleFetchFromLocal {
        map: args
            .module
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    });
    let environment = core::vm::Environment::new(module_fetch).await?;
    let app_state = AppState {
        dao: None,
        environment,
    };

    crate::app::http_entry::make_serve(&args.bind, app_state.clone()).await?;
    Ok(())
}
