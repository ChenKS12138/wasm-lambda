use std::sync::Arc;

use app::infra::AppState;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

mod app;
mod cli;
mod core;
mod db;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EntryModuleQuery {
    module_env: Option<String>,
    version_digest_value: Option<String>,
    version_raw_value: Option<Vec<u8>>,
    version_precompile: Option<Vec<u8>>,
}

struct ModuleFetchFromDao {
    dao: Arc<db::dao::Dao>,
}

#[async_trait::async_trait]
impl core::vm::FetchModule for ModuleFetchFromDao {
    async fn fetch_module(
        &self,
        engine: Arc<wasmtime::Engine>,
        module_name: String,
        version_alias: String,
    ) -> anyhow::Result<core::vm::Module> {
        let record = sqlx::query_as!(
            EntryModuleQuery,
            r#"SELECT
    module.module_env,
    module_version.version_digest_value,
    module_version.version_raw_value,
    module_version.version_precompile
FROM
    module LEFT JOIN module_version ON module.module_id = module_version.module_id
    WHERE module.module_name = ?
    ORDER BY module_version.create_at DESC
    LIMIT 1
"#,
            module_name
        )
        .fetch_one(&self.dao.pool)
        .await?;

        let binary = record.version_raw_value.unwrap();

        let envs = record.module_env.unwrap();
        let envs: std::collections::HashMap<String, String> =
            serde_json::from_str(envs.as_str()).unwrap();
        let envs: Vec<(String, String)> = envs.into_iter().map(|(k, v)| (k, v)).collect();

        let module = if let Some(precompile) = record.version_precompile {
            unsafe { wasmtime::Module::deserialize(&engine, &precompile) }?
        } else {
            wasmtime::Module::new(&engine, &binary)?
        };

        Ok(core::vm::Module {
            envs,
            module,
            module_name,
            version_alias,
            version_digest: record.version_digest_value.unwrap_or_default(),
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = sqlx::MySqlPool::connect("mariadb://local:local@10.211.55.14:3306/db").await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    let dao = Arc::new(db::dao::Dao::new(db));

    let module_fetch = ModuleFetchFromDao { dao: dao.clone() };

    let environment = core::vm::Environment::new(Box::new(module_fetch)).await?;
    let app_state = AppState { dao, environment };

    let (task1, task2) = tokio::join!(
        app::external_control::make_serve(app_state.clone()),
        app::http_entry::make_serve(app_state.clone()),
    );
    task1?;
    task2?;
    Ok(())
}
