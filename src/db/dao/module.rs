use crate::db::model::Module;



impl super::Dao {
    pub async fn create_module(&self,name:String,env:serde_json::Value) -> anyhow::Result<()>  {
        sqlx::query!(
            r#"
            INSERT INTO module (
                module_name,
                module_env
            ) VALUES (
                ?,
                ?
            )
            "#,
            name,
            env
        ).execute(&self.pool).await?;
        Ok(())
    }
    // pub async fn get_module_by_id(&self,id:u32) -> anyhow::Result<Option<Module>>  {
    //     let module = sqlx::query_as!(
    //         Module,
    //         r#"
    //         SELECT
    //             module_id,
    //             module_name,
    //             module_env
    //         FROM module
    //         WHERE module_id = ?
    //         "#,
    //         id
    //     ).fetch_optional(&self.pool).await?;
    //     Ok(module)
    // }
}