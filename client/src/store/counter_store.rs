use serde::{Deserialize, Serialize};
use yewdux::Store;

#[derive(Clone, PartialEq, Eq, Store, Default, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct CounterStore {
    pub counter: i32,
}
