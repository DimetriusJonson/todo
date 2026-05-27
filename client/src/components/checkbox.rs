use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct CallbackInfo {
    pub name: String,
    pub value: bool,
    pub target: HtmlInputElement,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,

    #[prop_or_default]
    pub title: String,

    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub value: bool,

    #[prop_or_else(default_onchange)]
    pub onchange: Callback<CallbackInfo>,
}

fn default_onchange() -> Callback<CallbackInfo> {
    Callback::from(|_: CallbackInfo| {})
}

#[function_component]
pub fn Checkbox(props: &Props) -> Html {
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

    let id = props.name.to_owned().to_lowercase().replace(" ", "");

    html! {
        <label class={format!("b-checkbox checkbox {}", props.class)}>
            <input type="checkbox" {id} name={props.name.to_owned()} checked={props.value.to_owned()} {onchange}/>
            <span class="check is-warning"></span>
        </label>
    }
}
