use std::sync::Arc;

use app::infra::AppState;

mod app;
mod core;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = sqlx::MySqlPool::connect("mariadb://local:local@10.211.55.14:3306/db").await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    let dao = Arc::new(db::dao::Dao::new(db));
    let environment = Arc::new(core::vm::Environment::new()?);
    let app_state = AppState { dao, environment };

    let (task1, task2) = tokio::join!(
        app::external_control::make_serve(app_state.clone()),
        app::http_entry::make_serve(app_state.clone()),
    );
    task1?;
    task2?;
    Ok(())
}
