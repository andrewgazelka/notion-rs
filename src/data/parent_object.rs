use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Data {
    PageId(String),
    DatabaseId(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParentObject {
    pub r#type: String,
    #[serde(flatten)]
    pub data: Data,
}
