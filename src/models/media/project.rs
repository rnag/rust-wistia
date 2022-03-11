use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub hashed_id: String,
    pub id: u64,
    pub name: String,
}
