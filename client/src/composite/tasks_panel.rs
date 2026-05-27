use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::{use_dispatch, use_store};

use crate::{
    api::task::{api_save_task, api_tasks},
    common::api_error::ApiError,
    components::checkbox::{CallbackInfo, Checkbox},
    composite::message_banner::{MessageStore, show_error, show_info},
    model::task::Task,
    routes::Route,
    store::settings_store::{Filter, Settings, SortKind},
};

#[component]
pub fn TasksPanel() -> Html {
    let message_dispatch = use_dispatch::<MessageStore>();
    let tasks = use_mut_ref(Vec::new);
    let (settings, settings_dispatch) = use_store::<Settings>();

    {
        let tasks = tasks.clone();
        let settings_dispatch = settings_dispatch.clone();
        let message_dispatch = message_dispatch.clone();
        use_effect_with((), move |_| {
            api_tasks(settings_dispatch, move |res| match res {
                Ok(data) => {
                    tasks.borrow_mut().clear();
                    tasks.borrow_mut().extend(data);
                }
                Err(err) => match err {
                    ApiError::UnAuthorized(_) => (),
                    _ => show_error(&message_dispatch, &err.to_string()),
                },
            })
        });
    }

    let completed_onchange = {
        let tasks = tasks.clone();
        Callback::from({
            move |info: CallbackInfo| {
                info.target.set_checked(!info.value);
                let settings_dispatch = settings_dispatch.clone();
                let message_dispatch = message_dispatch.clone();
                let tasks = tasks.clone();

                let mut patch = Task::new();
                patch.id = Some(
                    info.name[info.name.find('_').unwrap() + 1..]
                        .parse::<i32>()
                        .unwrap(),
                );
                patch.set_completed(info.value);

                api_save_task(patch, settings_dispatch, {
                    move |create_result| match create_result {
                        Ok(saved_task) => {
                            info.target.set_checked(saved_task.completed_at().is_some());
                            tasks
                                .borrow_mut()
                                .iter_mut()
                                .filter(|local_task| saved_task.id == local_task.id)
                                .for_each(|t| t.set_completed(saved_task.completed_at().is_some()));

                            show_info(&message_dispatch, "Задача сохранена.");
                        }
                        Err(err) => show_error(&message_dispatch, &err.to_string()),
                    }
                });
            }
        })
    };

    tasks
        .borrow_mut()
        .sort_by(|task1, task2| sort_task(task1, task2, &settings.sort_kind));

    let tasks_html = build_tasks_html(tasks, settings, completed_onchange);

    html! {
        <table class="table is-striped is-fullwidth">
            <thead>
                <tr>
                    <th>{"Приоритет"}</th>
                    <th>{"Завершена"}</th>
                    <th>{"Название"}</th>
                    <th class="is-hidden-mobile">{"Описание"}</th>
                </tr>
            </thead>
            <tbody>
                if !tasks_html.is_empty() {
                    { tasks_html }
                } else {
                    { html! { <tr><td colspan="3" style="text-align: center;">{"Нет записей"}</td></tr> } }
                }
            </tbody>
        </table>

    }
}

fn build_tasks_html(
    tasks: Rc<RefCell<Vec<Task>>>,
    settings: Rc<Settings>,
    completed_onchange: Callback<CallbackInfo>,
) -> Vec<Html> {
    tasks.borrow().iter().filter(|task| filter_task(task, &settings.filter)).map(|task| {
        html! {
            <tr>
                <td class={priority_style(task)}>{task.priority_name()}</td>
                <td>
                    <Checkbox class="is-medium" name={format!("completed_{}", task.id.unwrap())} value={task.completed_at().is_some()} onchange={completed_onchange.clone()}
                    title={match task.completed_at() {
                        Some(completed_at) => {format!("{}", completed_at.format("%d.%m.%Y %H:%M"))},
                        _ => {"".to_owned()},
                    }}/>
                </td>
                <td><Link<Route> to={Route::Task { id: {task.id.unwrap()} }}>{ task.title.to_owned().unwrap_or_default() }</Link<Route>></td>
                <td class="is-hidden-mobile">{task.description.to_owned().unwrap_or_default()}</td>
            </tr>
        }
    }).collect()
}

fn priority_style(task: &Task) -> String {
    match &task.priority {
        Some(priority) => {
            let p = priority.as_str();
            match p {
                "C" => "critical",
                "H" => "high",
                "N" => "normal",
                _ => "low",
            }
        }
        _ => "gray",
    }
    .to_owned()
}

fn filter_task(task: &Task, filter: &Filter) -> bool {
    match filter {
        Filter::Empty => true,
        Filter::Completed => task.completed_at().is_some(),
        Filter::Uncompleted => task.completed_at().is_none(),
    }
}

fn sort_task(task1: &Task, task2: &Task, sort_kind: &SortKind) -> Ordering {
    match sort_kind {
        SortKind::Empty => task1.id.cmp(&task2.id),
        SortKind::Title => task1.title.cmp(&task2.title),
        SortKind::Priority => task1.priority_name().cmp(&task2.priority_name()),
    }
}
