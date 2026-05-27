use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::checkbox::CallbackInfo;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub label: String,
    pub value: bool,

    #[prop_or_default]
    pub class: String,

    #[prop_or_else(default_onchange)]
    pub onchange: Callback<CallbackInfo>,
}

fn default_onchange() -> Callback<CallbackInfo> {
    Callback::from(|_: CallbackInfo| {})
}

#[component]
pub fn CheckboxWithLabel(props: &Props) -> Html {
    let id = props.name.to_owned().to_lowercase().replace(" ", "");

    let onchange = {
        let onchange = props.onchange.clone();
        let name = props.name.to_owned();
        Callback::from(move |event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            onchange.emit(CallbackInfo {
                name: name.to_owned(),
                value: target.checked(),
                target,
            });
        })
    };

    html! {
        <div class="control">
            <label class={format!("b-checkbox checkbox {}", props.class)}>
                <input type="checkbox" {id} name={props.name.to_owned()} checked={props.value.to_owned()} {onchange}/>
                <span class="check is-warning"></span>
                <span class="control-label">{props.label.to_owned()}</span>
            </label>
        </div>
    }
}
