use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    
    #[prop_or_default]
    pub class: String,

    pub label: String,
    
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub loading: bool,

    #[prop_or_else(default_onclick)]
    pub onclick: Callback<MouseEvent>,
}

fn default_onclick() -> Callback<MouseEvent> {
    Callback::from(|_: MouseEvent| {})
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    let onmouseup = Callback::from(move |e: MouseEvent| {
        if let Some(target) = e.target() {
            let btn = target.unchecked_into::<HtmlButtonElement>();
            btn.blur().unwrap();
        }
    });

    let loading_class = if props.loading {
        " is-loading"
    } else {
        ""
    };
    

    html! {
        <button id={props.id.to_owned()} class={format!("button is-rounded{} {}", loading_class, props.class)} onclick={props.onclick.clone()} role="button" {onmouseup} disabled={props.disabled}> {&props.label} </button>
    }
}

