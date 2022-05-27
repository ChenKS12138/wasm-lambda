use crate::db::model::ModuleOwner;

impl super::Dao {
    pub async fn create_module_owner(&self,name:String) -> anyhow::Result<()>  {
        sqlx::query!(
            r#"
            INSERT INTO module_owner (
                module_owner_name
            ) VALUES (
                ?
            )
            "#,
            name
        ).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn select_module_owner(&self) -> anyhow::Result<Vec<ModuleOwner>> {
        Ok(sqlx::query_as!(
            ModuleOwner,
            r#"
            SELECT
                module_owner_id,
                module_owner_name
            FROM
                module_owner
            "#
        ).fetch_all(&self.pool).await?)
    }
}

