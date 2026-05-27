use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use yew::prelude::*;
use yew_router::{hooks::use_navigator, prelude::Navigator};
use yewdux::{Dispatch, use_dispatch, use_store};

use crate::{
    api::task::{api_get_task, api_save_task},
    common::{api_error::ApiError, ui_helper::get_validate_error},
    components::{
        button::{Button}, checkbox::CallbackInfo, main_title::{MainTitle}, select_input::SelectOptions, text_with_error::TextWithError, textarea::TextArea
    },
    composite::{
        checkbox_with_label::CheckboxWithLabel,
        message_banner::{MessageBanner, MessageStore, show_error, show_info},
        select_with_label::SelectWithLabel,
    },
    model::task::Task,
    routes::Route,
    store::settings_store::Settings,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<i32>,
    pub title: String,
}

#[component]
pub fn TaskEditPage(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let message_dispatch = use_dispatch::<MessageStore>();
    let errors = use_state(BTreeMap::<String, String>::new);
    let (settings, settings_dispatch) = use_store::<Settings>();

    if settings.user.is_none() {
        navigator.push(&Route::Login);
        return html! {};
    }
    let old_task_state = use_state(Task::new);

    let task_state = use_mut_ref(Task::new);

    use_effect_with((), {
        let old_task_state = old_task_state.clone();
        let task_state = task_state.clone();
        let task_id = props.id;
        let message_dispatch = message_dispatch.clone();
        let navigator = navigator.clone();
        let settings_dispatch = settings_dispatch.clone();
        move |_| {
            if let Some(task_id) = task_id {
                api_get_task(task_id, settings_dispatch, move |res| match res {
                    Ok(task) => {
                        task_state.borrow_mut().set_from_task(&task);
                        old_task_state.set(task);
                    }
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
        }
    });

    let title_change_handler: Callback<String> = {
        let task_state = task_state.clone();
        Callback::from(move |value: String| task_state.borrow_mut().set_title(value))
    };

    let priority_onchange = {
        let task_state = task_state.clone();
        Callback::from(move |value: String| task_state.borrow_mut().set_priority(value))
    };

    let description_onchange = {
        let task_state = task_state.clone();
        Callback::from(move |value: String| task_state.borrow_mut().set_description(value))
    };

    let completed_onchange = {
        let task_state = task_state.clone();
        Callback::from(move |info: CallbackInfo| task_state.borrow_mut().set_completed(info.value))
    };

    let onsubmit = {
        let navigator = navigator.clone();
        let task_state = task_state.clone();
        let errors = errors.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            do_save_task(
                navigator.clone(),
                message_dispatch.clone(),
                settings_dispatch.clone(),
                old_task_state.clone(),
                task_state.clone(),
                errors.clone(),
            );
        })
    };

    let cancel_onclick = {
        let id = props.id;
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(id) = id {
                navigator.push(&Route::Task { id });
            } else {
                navigator.push(&Route::Home);
            }
        })
    };

    let priorities = SelectOptions::new(Task::priorities());

    html! {
        <section class="section">
            <div class="container">
                <MainTitle title={props.title.to_owned()}/>
                <form {onsubmit}>
                    <fieldset disabled={settings.api_in_progress}>
                        <div class="level">
                            <div class="level-left">
                                <div class="level-item">
                                    <SelectWithLabel name="priority" label={"Приоритет:"} value={task_state.borrow().priority.to_owned().unwrap_or_default()} options={priorities} onchange={priority_onchange} />
                                </div>
                            </div>

                            <div class="level-right">
                                <div class="level-item">
                                    <CheckboxWithLabel name="completed" label="Завершена" value={task_state.borrow().completed_at().is_some()} onchange={completed_onchange}/>
                                </div>   
                            </div>
                        </div>

                        <div class="field">
                            <TextWithError name="title" placeholder="Название" value={task_state.borrow().title.to_owned().unwrap_or_default()} error={get_validate_error("title", &errors)} onchange={title_change_handler}/>
                        </div>
                        <div class="field">
                            <TextArea name="description" placeholder="Описание" value={task_state.borrow().description.to_owned()} onchange={description_onchange}/>
                        </div>
                        <div class="field is-grouped">
                            <div class="control">
                                <Button class="is-primary" label="Сохранить" loading={settings.api_in_progress}/>
                            </div>
                            <div class="control">
                                <Button class="is-light" label="Отмена" onclick={cancel_onclick} />
                            </div>
                        </div>
                    </fieldset>
                </form>
            </div>
            <MessageBanner/>
        </section>
    }
}

fn do_save_task(
    navigator: Navigator,
    message_dispatch: Dispatch<MessageStore>,
    settings_dispatch: Dispatch<Settings>,
    old_task_state: UseStateHandle<Task>,
    task_state: Rc<RefCell<Task>>,
    errors: UseStateHandle<BTreeMap<String, String>>,
) {
    let patch = Task::create_patch(&old_task_state, &task_state.borrow());
    api_save_task(patch, settings_dispatch, {
        let errors = errors.clone();
        move |create_result| match create_result {
            Ok(saved_task) => {
                show_info(&message_dispatch, "Задача сохранена.");
                navigator.push(&Route::Task {
                    id: saved_task.id.unwrap(),
                });
            }
            Err(err) => match err {
                ApiError::Validation(_, validate_errors) => {
                    errors.set(validate_errors);
                }
                _ => show_error(&message_dispatch, &err.to_string()),
            },
        }
    });
}
