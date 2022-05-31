mod app;
mod cmd;
mod core;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cmd::boost().await?;
    Ok(())
}
