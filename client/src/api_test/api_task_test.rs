use std::collections::BTreeMap;

use chrono::Utc;
use gloo::timers::callback::Timeout;
use yewdux::Dispatch;

use crate::{common::api_error::ApiError, model::task::Task, store::settings_store::Settings};

fn test_tasks() -> BTreeMap<i32, Task> {
    let mut map = BTreeMap::new();
    let now = Utc::now();

    map.insert(1, Task {
            id: Some(1),
            priority: Some("L".to_owned()),
            title: "Приготовить завтрак".to_owned(),
            ..Default::default()
        });
    map.insert(2, Task {
            id: Some(2),
            priority: Some("H".to_owned()),
            title: "Сделать задачу по работе".to_owned(),
            completed_at: Some(now),
            ..Default::default()
        });
    map.insert(3, Task {
            id: Some(3),
            priority: Some("N".to_owned()),
            title: "Не забыть вынести мусор".to_owned(),
            ..Default::default()
        });
    map.insert(4, Task {
            id: Some(4),
            priority: Some("L".to_owned()),
            title: "Позвонить в МТС".to_owned(),
            completed_at: Some(now),
            ..Default::default()
        });
    map.insert(5, Task {
            id: Some(5),
            priority: Some("C".to_owned()),
            title: "Закрыть теплицу пока не высохли овози от мороза".to_owned(),
            ..Default::default()
        });
    map
}

pub fn api_tasks<F>(_settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<Vec<Task>, ApiError>) + 'static,
{
    f(Ok(test_tasks().into_values().collect()))
}

pub fn api_save_task<F>(task: &Task, settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<Task, ApiError>) + 'static,
{
    settings_dispatch.reduce_mut(|s| s.api_in_progress = true);
    let task = task.clone();
    Timeout::new(1000, move || {
        settings_dispatch.reduce_mut(|s| s.api_in_progress = false);
        if task.title == "aaa" {
            f(Err(ApiError::other("Task already exists!")))
        } else {
            let mut task = task.clone();
            task.id = Some(1);
            f(Ok(task))
        }
    })
    .forget();
}

pub fn api_get_task<F>(id: i32, settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<Task, ApiError>) + 'static,
{
    settings_dispatch.reduce_mut(|s| s.api_in_progress = true);

    Timeout::new(1000, move || {
        settings_dispatch.reduce_mut(|s| s.api_in_progress = false);
        if let Some(task) = test_tasks().get(&id) {
            f(Ok(task.clone()))
        } else {
            f(Err(ApiError::other("Not found task!")))
        }
    })
    .forget();
}
