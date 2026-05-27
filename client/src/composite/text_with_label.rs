use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::text_input::{TextInputSize, TextInputType};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub label: String,
    pub value: String,

    #[prop_or_default]
    pub size: TextInputSize,

    #[prop_or_default]
    pub input_type: TextInputType,

    #[prop_or_default]
    pub focus: bool,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_else(default_onchange)]
    pub onchange: Callback<String>,
}

fn default_onchange() -> Callback<String> {
    Callback::from(|_: String| {})
}

#[component]
pub fn TextWithLabel(props: &Props) -> Html {
    let input_ref = use_node_ref();
    if props.focus
        && let Some(input) = input_ref.cast::<HtmlInputElement>()
    {
        input.focus().unwrap();
    }

    let onchange = {
        let onchange = props.onchange.clone();
        Callback::from(move |event: Event| {
            onchange.emit(event.target_unchecked_into::<HtmlInputElement>().value());
        })
    };

    html! {
        <>
            <label class="label">{&props.label}</label>
            <div class="control">
                <input ref={input_ref} class="input" type={props.input_type.to_string()} id={props.name.to_owned()} name={props.name.to_owned()}
                    value={props.value.to_owned()} placeholder={props.placeholder.to_owned()} {onchange}/>
            </div>
        </>
    }
}
