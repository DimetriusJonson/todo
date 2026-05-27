use std::{collections::BTreeMap, ops::Deref};

use yew::{Callback, SubmitEvent, UseStateHandle};

pub struct UiCallback<V> {
    callback: Callback<V>,
}

impl<V> UiCallback<V> {
    pub fn new<T, F>(state_handle: &UseStateHandle<T>, f: F) -> Self
    where
        T: Clone + 'static,
        F: Fn(&mut T, V) + 'static,
    {
        let state_handle_cloned = state_handle.clone();
        let callback: Callback<V> = Callback::from(move |value: V| {
            let mut state = state_handle_cloned.deref().clone();
            f(&mut state, value);

            state_handle_cloned.set(state);
        });

        Self { callback }
    }

    pub fn get(&self) -> &Callback<V> {
        &self.callback
    }
}

pub fn create_submit_event<T>(
    callback: &Callback<T>,
    state: &UseStateHandle<T>,
) -> Callback<SubmitEvent>
where
    T: Clone + 'static,
{
    let cloned_callback = callback.clone();
    let cloned_state = state.clone();
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        cloned_callback.emit(cloned_state.deref().clone());
    })
}

pub fn get_validate_error(name: &str, errors: &UseStateHandle<BTreeMap<String, String>>) -> String {
    errors.get(name).unwrap_or(&"".to_owned()).to_owned()
}

