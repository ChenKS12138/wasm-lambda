use std::{sync::Arc, time::Duration};

use notify::{RecommendedWatcher, Watcher};
use wasmtime::Engine;

use super::DevArgs;
use crate::{app::infra::AppState, core};

// TODO improve performance
struct ModuleFetchFromLocal {
    pre_compiled: std::collections::HashMap<String, Arc<tokio::sync::Mutex<Vec<u8>>>>,
}

impl ModuleFetchFromLocal {
    pub fn new(engine: Arc<Engine>, raw: std::collections::HashMap<String, String>) -> Self {
        let mut pre_compiled = std::collections::HashMap::new();
        for (k, v) in raw {
            let value = Arc::new(tokio::sync::Mutex::new(vec![]));
            let engine = engine.clone();
            let value_clone = value.clone();
            let k_clone = k.clone();
            tokio::spawn(async move {
                println!("precompile {}...", k_clone);
                let engine = engine.clone();
                let value_guard = value.clone();
                let mut value = value_guard.lock().await;
                let content = tokio::fs::read(&v).await.unwrap();
                *value = engine.precompile_module(&content).unwrap();
                println!("precompile {} done!", k_clone);
                let (watcher_tx, mut watcher_rx) = tokio::sync::mpsc::channel(1);

                let value_guard = value_guard.clone();
                let v_clone = v.clone();
                tokio::spawn(async move {
                    let v_clone = v_clone.clone();
                    while let Some(_) = watcher_rx.recv().await {
                        println!("recompile {}...", v_clone);
                        let mut value = value_guard.lock().await;
                        let content = std::fs::read(&v_clone).unwrap();
                        *value = engine.precompile_module(&content).unwrap();
                        println!("recompile {} done!", v_clone);
                    }
                });

                let v_clone = v.clone();
                std::thread::spawn(move || {
                    let (change_tx, change_rx) = std::sync::mpsc::channel();
                    let mut watcher: RecommendedWatcher =
                        Watcher::new(change_tx, Duration::from_secs(1)).unwrap();

                    watcher
                        .watch(v_clone, notify::RecursiveMode::NonRecursive)
                        .unwrap();

                    while let Ok(event) = change_rx.recv() {
                        match event {
                            notify::DebouncedEvent::NoticeWrite(_)
                            | notify::DebouncedEvent::NoticeRemove(_)
                            | notify::DebouncedEvent::Rescan
                            | notify::DebouncedEvent::Create(_)
                            | notify::DebouncedEvent::Chmod(_)
                            | notify::DebouncedEvent::Remove(_)
                            | notify::DebouncedEvent::Rename(_, _)
                            | notify::DebouncedEvent::Error(_, _) => {}

                            notify::DebouncedEvent::Write(_) => {
                                let _ = watcher_tx.blocking_send(());
                            }
                        }
                    }
                })
            });
            pre_compiled.insert(k, value_clone.clone());
        }
        Self { pre_compiled }
    }
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
        match self.pre_compiled.get(&module_name) {
            None => Err(anyhow::anyhow!("module not found")),
            Some(content) => {
                let content = content.lock().await;
                let internal_module = unsafe { wasmtime::Module::deserialize(&engine, &*content) }?;
                Ok(core::vm::Module {
                    module_name: module_name.clone(),
                    envs: vec![],
                    version_digest: "not available in local".to_string(),
                    module: internal_module,
                    version_alias: "".to_string(),
                })
            }
        }
    }
}

pub async fn dev(args: DevArgs) -> anyhow::Result<()> {
    let engine = core::vm::Environment::new_engine()?;

    let module_fetch = Box::new(ModuleFetchFromLocal::new(
        engine.clone(),
        args.module.into_iter().collect(),
    ));
    let environment = core::vm::Environment::new(module_fetch, engine.clone()).await?;
    let app_state = AppState {
        dao: None,
        environment,
    };

    crate::app::http_entry::make_serve(&args.bind, app_state.clone()).await?;
    Ok(())
}
