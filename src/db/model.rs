use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct ModuleOwner {
    pub module_owner_id: u64,
    pub module_owner_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Module {
    pub module_id:u64,
    pub module_name:String,
    pub env: sqlx::types::Json<HashMap<String,String>>,
    pub module_owner_id: u64,
    pub module_owner_version:Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct ModelVersion {
    pub model_version_id: u64,
    pub hash_value: String,
    pub raw_value: Vec<u8>,
    pub pre_compiled: Option<Vec<u8>>,
    pub module_id: u64,
}