use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use yew::prelude::*;
use yew_router::{hooks::use_navigator, prelude::Navigator};
use yewdux::{Dispatch, use_dispatch, use_store};

use crate::{
    api::user::api_login,
    common::{api_error::ApiError, ui_helper::get_validate_error},
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
pub fn LoginPage() -> Html {
    let message_dispatch = use_dispatch::<MessageStore>();
    let (settings, settings_dispatch) = use_store::<Settings>();
    let navigator = use_navigator().unwrap();
    let user_state = use_mut_ref(User::default);
    let errors = use_state(BTreeMap::<String, String>::new);
    let pass_auto_focus = use_state(|| false);

    use_effect_with((), {
        let user_state = user_state.clone();
        let settings_dispatch = settings_dispatch.clone();
        let pass_auto_focus = pass_auto_focus.clone();
        move |_| {
            if let Some(user) = &settings_dispatch.get().user {
                user_state
                    .borrow_mut()
                    .set_username(user.username.to_owned());
                pass_auto_focus.set(true);
            }
        }
    });

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

            do_login(
                navigator.clone(),
                message_dispatch.clone(),
                settings_dispatch.clone(),
                user_state.clone(),
                errors.clone(),
            );
        })
    };

    html! {
        <section class="section">
            <div class="container">
                <MainTitle title="Вход в систему"/>
                <form class="box" {onsubmit}>
                    <fieldset disabled={settings.api_in_progress}>
                        <div class="field">
                            <TextWithError name="userName" placeholder="Имя пользователя" value={user_state.borrow().username.to_owned()} error={get_validate_error("username", &errors)} onchange={username_change_handler}/>
                        </div>
                        <div class="field">
                            <TextWithError name="password" placeholder="Пароль" value={""} input_type={TextInputType::Password} error={get_validate_error("password", &errors)} onchange={password_change_handler} focus={*pass_auto_focus}/>
                        </div>
                        <div class="field">
                            <div class="control"><Button class="is-primary" label="Войти" loading={settings.api_in_progress}/></div>
                        </div>
                    </fieldset>
                </form>
            </div>
            <MessageBanner/>
        </section>
    }
}

fn do_login(
    navigator: Navigator,
    message_dispatch: Dispatch<MessageStore>,
    settings_dispatch: Dispatch<Settings>,
    user_state: Rc<RefCell<User>>,
    errors: UseStateHandle<BTreeMap<String, String>>,
) {
    api_login(&user_state.borrow(), settings_dispatch.clone(), {
        let errors = errors.clone();

        move |login_result| {
            match login_result {
                Ok(authorized_user) => {
                    settings_dispatch
                        .reduce_mut(move |settings| settings.user = Some(authorized_user));
                    navigator.push(&Route::Home);
                    show_info(&message_dispatch, "Вы вошли!");
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
