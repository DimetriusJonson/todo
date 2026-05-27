use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::use_dispatch;

use crate::{
    api::user::api_logout,
    common::api_error::ApiError,
    composite::message_banner::{MessageStore, show_error, show_info},
    routes::Route,
    store::settings_store::Settings,
};

#[function_component]
pub fn LogoutPage() -> Html {
    let settings_dispatch = use_dispatch::<Settings>();
    let message_dispatch = use_dispatch::<MessageStore>();
    let navigator = use_navigator().unwrap();

    api_logout(settings_dispatch, move |res| {
        match res {
            Ok(_) => show_info(&message_dispatch, "Вы вышли!"),
            Err(err) => {
                match err {
                    ApiError::UnAuthorized(_) => (),
                    _ => show_error(&message_dispatch, &err.to_string()),
                }
            }
        }
        navigator.push(&Route::Home);
    });

    html! {}
}
