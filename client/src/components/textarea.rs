use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,

    #[prop_or_default]
    pub value: Option<String>,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_else(default_onchange)]
    pub onchange: Callback<String>,
}

fn default_onchange() -> Callback<String> {
    Callback::from(|_: String| {})
}

#[function_component]
pub fn TextArea(props: &Props) -> Html {
    let onchange = {
        let onchange = props.onchange.clone();
        Callback::from(move |event: Event| {
            onchange.emit(event.target_unchecked_into::<HtmlInputElement>().value());
        })
    };

    html! {
        <textarea class="textarea" rows="4" cols="50" id={props.name.to_owned()} name={props.name.to_owned()}
            value={props.value.to_owned()} placeholder={props.placeholder.to_owned()} {onchange}/>
    }
}
