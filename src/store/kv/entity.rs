use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KV {
    pub key: String,
    pub value: String,
}