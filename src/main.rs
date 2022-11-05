#[macro_use]
extern crate log;

mod app;
mod cmd;
mod core;

const LOG_LEVEL_ENV: &str = "RUST_LOG";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize Log Library
    if !std::env::var(LOG_LEVEL_ENV).is_ok() {
        std::env::set_var(LOG_LEVEL_ENV, "info");
    }
    env_logger::init_from_env(LOG_LEVEL_ENV);

    cmd::boost().await?;

    Ok(())
}
