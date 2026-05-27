use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    pub title: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn MainTitle(props: &Props) -> Html {
    html! {
        <h1 class={format!("title {}", props.class)}>
            { &props.title }
        </h1>
    }
}
