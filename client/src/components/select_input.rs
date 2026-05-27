use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;

pub type SelectOption = (String, String);

#[derive(PartialEq, Clone)]
pub struct SelectOptions {
    inner: Rc<Vec<SelectOption>>,
}

impl SelectOptions {
    pub fn new(options: Vec<SelectOption>) -> Self {
        Self {
            inner: Rc::new(options)
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub options: SelectOptions,

    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub value: String,

    pub not_selected_text: String,

    #[prop_or_else(default_onchange)]
    pub onchange: Callback<String>,
}

fn default_onchange() -> Callback<String> {
    Callback::from(|_: String| {})
}

#[function_component]
pub fn SelectInput(props: &Props) -> Html {
    let onchange = {
        let onchange = props.onchange.clone();
        Callback::from(move |event: Event| {
            onchange.emit(event.target_unchecked_into::<HtmlInputElement>().value());
        })
    };

    let id = props.name.to_owned().to_lowercase().replace(" ", "");

    html! {
        <div class={format!("select {}", props.class)}>
            <select {id} name={props.name.to_owned()} {onchange}>
                <option value={""} selected={props.value.is_empty()}>{&props.not_selected_text}</option>
                { props.options.inner.iter().map(|val| {
                    html! {
                        <option value={val.0.to_owned()} selected={*val.0 == props.value} >{val.1.to_owned()}</option>
                    }
                    }).collect::<Html>()
                }
            </select>
        </div>   
    }
}
