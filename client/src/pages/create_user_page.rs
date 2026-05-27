use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use yew::prelude::*;
use yew_router::{hooks::use_navigator, prelude::Navigator};
use yewdux::{Dispatch, use_dispatch, use_store};

use crate::{
    api::user::api_create_user,
    common::{api_error::{ApiError}, ui_helper::get_validate_error},
    components::{
        button::Button,
        main_title::{MainTitle},
        text_input::TextInputType,
        text_with_error::TextWithError,
    },
    composite::message_banner::{MessageBanner, MessageStore, show_error, show_info},
    model::user::User,
    routes::Route,
    store::settings_store::Settings,
};

#[component]
pub fn CreateUserPage() -> Html {
    let message_dispatch = use_dispatch::<MessageStore>();
    let user_state = use_mut_ref(User::default);
    let (settings, settings_dispatch) = use_store::<Settings>();
    let navigator = use_navigator().unwrap();
    let errors = use_state(BTreeMap::<String, String>::new);

    let username_change_handler: Callback<String> = {
        let user_state = user_state.clone();
        Callback::from(move |value: String| user_state.borrow_mut().set_username(value))
    };

    let password_change_handler: Callback<String> = {
        let user_state = user_state.clone();
        Callback::from(move |value: String| user_state.borrow_mut().set_password(value))
    };

    let onsubmit = {
        let user_state = user_state.clone();
        let errors = errors.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            do_create_user(
                navigator.clone(),
                message_dispatch.clone(),
                settings_dispatch.clone(),
                user_state.clone(),
                errors.clone(),
            );
        })
    };

    html! {
        <section class="section container">
            <MainTitle title="Создать пользователя"/>
            <form class="box" {onsubmit}>
                <fieldset disabled={settings.api_in_progress}>
                    <div class="field"><TextWithError name="userName" placeholder="Имя пользователя" value={user_state.borrow().username.to_owned()} error={get_validate_error("username", &errors)} onchange={username_change_handler}/></div>
                    <div class="field"><TextWithError name="password" placeholder="Пароль" input_type={TextInputType::Password} value={""} error={get_validate_error("password", &errors)} onchange={password_change_handler}/></div>
                    <div class="field"><div class="control"><Button class="is-primary" label="Создать" loading={settings.api_in_progress}/></div></div>
                </fieldset>
            </form>
            <MessageBanner/>
        </section>
    }
}

fn do_create_user(
    navigator: Navigator,
    message_dispatch: Dispatch<MessageStore>,
    settings_dispatch: Dispatch<Settings>,
    user_state: Rc<RefCell<User>>,
    errors: UseStateHandle<BTreeMap<String, String>>,
) {
    api_create_user(user_state.borrow().clone(), settings_dispatch.clone(),{
        let errors = errors.clone();
        move |create_result| {
            match create_result {
                Ok(new_user) => {
                    let navigator = navigator.clone();
                    let message_dispatch = message_dispatch.clone();
                    settings_dispatch.reduce_mut(move |settings| {
                        settings.user = Some(new_user);
                        show_info(&message_dispatch, "Пользователь успешно создан.");
                        navigator.push(&Route::Login)
                    });
                }
                Err(err) => match err {
                    ApiError::Validation(_, validate_errors) => {
                        errors.set(validate_errors);
                    }
                    _ => show_error(&message_dispatch, &err.to_string()),
                },
            }
        }
    });
}
