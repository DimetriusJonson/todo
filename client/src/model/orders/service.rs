use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Service {
    pub id: u64,
    pub name: String,
}