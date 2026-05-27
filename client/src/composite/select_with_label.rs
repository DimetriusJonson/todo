use yew::prelude::*;

use crate::components::select_input::{SelectInput, SelectOptions};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub label: String,
    pub value: String,
    pub options: SelectOptions,

    #[prop_or_else(default_onchange)]
    pub onchange: Callback<String>,
}

fn default_onchange() -> Callback<String> {
    Callback::from(|_: String| {})
}

#[component]
pub fn SelectWithLabel(props: &Props) -> Html {
    let id = props.name.to_owned().to_lowercase().replace(" ", "");

    html! {
        <>
            <label class="label mx-2" for={id.to_owned()}>{ props.label.to_owned() }</label>
            <SelectInput name={id} value={props.value.to_owned()} options={props.options.clone()} not_selected_text={"Не выбран"} onchange={props.onchange.clone()} />
        </>
    }
}
