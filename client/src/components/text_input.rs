use std::fmt::Display;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Default, Clone)]
pub enum TextInputSize {
    #[default]
    Normal,
    Big,
}

impl Display for TextInputSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextInputSize::Normal => write!(f, "normal"),
            TextInputSize::Big => write!(f, "big"),
        }
    }
}

#[derive(PartialEq, Default, Clone)]
pub enum TextInputType {
    #[default]
    Text,
    Password,
}

impl Display for TextInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextInputType::Text => write!(f, "text"),
            TextInputType::Password => write!(f, "password"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,

    #[prop_or_default]
    pub value: Option<String>,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub size: TextInputSize,

    #[prop_or_default]
    pub input_type: TextInputType,

    #[prop_or_default]
    pub focus: bool,

    #[prop_or_else(default_onchange)]
    pub onchange: Callback<String>,
}

fn default_onchange() -> Callback<String> {
    Callback::from(|_: String| {})
}

#[function_component]
pub fn TextInput(props: &Props) -> Html {
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
        <input ref={input_ref} class="input" type={props.input_type.to_string()} id={props.name.to_owned()} name={props.name.to_owned()}
            value={props.value.to_owned()} placeholder={props.placeholder.to_owned()} {onchange}/>
    }
}
