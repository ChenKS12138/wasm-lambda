use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct ModuleOwner {
    pub module_owner_id: u32,
    pub module_owner_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Module {
    pub module_id:u32,
    pub module_name:String,
    pub module_env: Option<String>,
    pub module_owner_id: Option<u64>,
    pub module_owner_version:Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct ModelVersion {
    pub model_version_id: u32,
    pub hash_value: Option<String>,
    pub raw_value: Option<Vec<u8>>,
    pub pre_compiled: Option<Vec<u8>>,
    pub module_id: Option<u64>,
}