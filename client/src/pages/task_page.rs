
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::{use_dispatch, use_store};

use crate::{
    api::task::{api_delete_task, api_get_task},
    common::api_error::ApiError,
    components::button::{Button},
    composite::message_banner::{MessageBanner, MessageStore, show_error, show_info},
    model::task::Task,
    routes::Route,
    store::settings_store::Settings,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[component]
pub fn TaskPage(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let message_dispatch = use_dispatch::<MessageStore>();
    let (settings, settings_dispatch) = use_store::<Settings>();

    if settings.user.is_none() {
        navigator.push(&Route::Login);
        return html! {};
    }
    let task_state = use_mut_ref(Task::new);

    use_effect_with((), {
        let task_state = task_state.clone();
        let task_id = props.id;
        let message_dispatch = message_dispatch.clone();
        let navigator = navigator.clone();
        let settings_dispatch = settings_dispatch.clone();
        move |_| {
            api_get_task(task_id, settings_dispatch, move |res| match res {
                Ok(task) => task_state.borrow_mut().set_from_task(&task),
                Err(err) => {
                    show_error(&message_dispatch, &err.to_string());
                    if let ApiError::UnAuthorized(_) = err {
                        navigator.push(&Route::Login);
                    } else {
                        navigator.push(&Route::Home);
                    }
                }
            })
        }
    });

    let edit_onclick = Callback::from({
        let navigator = navigator.clone();
        let id = props.id;
        move |_: MouseEvent| navigator.push(&Route::TaskEdit { id })
    });

    let delete_onclick = Callback::from({
        let id = props.id;
        move |_: MouseEvent| {
            let settings_dispatch = settings_dispatch.clone();
            api_delete_task(id, settings_dispatch, {
                let navigator = navigator.clone();
                let message_dispatch = message_dispatch.clone();
                move |create_result| match create_result {
                    Ok(_) => {
                        show_info(&message_dispatch, "Задача удалена.");
                        navigator.push(&Route::Home);
                    }
                    Err(err) => {
                        show_error(&message_dispatch, &err.to_string());
                        if let ApiError::UnAuthorized(_) = err {
                            navigator.push(&Route::Login);
                        } else {
                            navigator.push(&Route::Home);
                        }
                    }
                }
            })
        }
    });

    html! {
        <section class="section">
            <div class="container">
                <div class="message">
                    <div class="message-header">
                        <p>{"Сделать"}</p>
                    </div>

                    <div class="message-body">


                        <div class="media">
                            <div class="media-left">
                                if task_state.borrow().completed_at().is_some() {
                                    <span class="is-size-3">{"✅"}</span>
                                } else {
                                    <span class="is-size-3">{"❌"}</span>
                                }
                            </div>
                            <div class="media-content">
                                <p class="title is-4">{task_state.borrow().title.to_owned().unwrap_or_default()}</p>
                                <p class="subtitle is-6">{task_state.borrow().priority_name()}</p>
                            </div>
                        </div>

                        <div class="content">
                            if let Some(description) = &task_state.borrow().description {
                                <p>{description.to_owned()}</p>
                            }
                        </div>

                        <div class="field is-grouped">
                            <Button class="is-light" label="Редактировать" onclick={edit_onclick} disabled={settings.api_in_progress}/>
                            <Button class="is-danger" label="Удалить" onclick={delete_onclick} disabled={settings.api_in_progress} loading={settings.api_in_progress}/>
                        </div>
                    </div>
                </div>
            </div>
            <MessageBanner/>
        </section>
    }
}
