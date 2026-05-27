use serde::{Deserialize, Serialize};

use crate::model::orders::service::Service;

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub agency_name: String,
    pub buyer_name: String,
    pub services: Vec<Service>,
}