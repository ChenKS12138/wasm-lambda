mod app;
mod core;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut db = sqlx::MySqlPool::connect("mariadb://local:local@10.211.55.2:3306/db").await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    
    // let mut instance = core::vm::Instance::new(|engine| -> anyhow::Result<Module> {
    //     let m = Module::from_file(&engine, "/home/cattchen/codes/github.com/ChenKS12138/wasm-lambda/target/wasm32-wasi/debug/hello-world.wasi.wasm")?;
    //     Ok(m)
    // })?;
    // let evt = message::TriggerEvent::EventHttpRequest(message::Request {
    //     path: "www.baidu.com".to_string(),
    //     headers: HashMap::new(),
    //     method: "GET".to_string(),
    //     body: None,
    // });
    // let resp = instance.run(evt).await?;
    // println!("{:?}", resp);
    // println!("end");
    let (task1,task2) = tokio::join!(
        app::external_control::make_serve(),
        app::http_entry::make_serve()
    );
    task1?;
    task2?;
    Ok(())
}
