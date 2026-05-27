
use serde::{Deserialize, Serialize};
use yewdux::Store;

use crate::model::orders::order::Order;

#[derive(Clone, PartialEq, Eq, Store, Default, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct OrderStore {
    pub order: Order,
}

