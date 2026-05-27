use serde::Deserialize;
use yewdux::Dispatch;

use crate::{
    api_real::api_common::{RequestMethod, make_request}, common::api_error::ApiError, model::task::Task, store::settings_store::Settings
};

pub fn api_tasks<F>(settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<Vec<Task>, ApiError>) + 'static,
{
    make_request::<TasksResponse, (), _>(
        RequestMethod::Get,
        "tasks".to_owned(),
        None,
        settings_dispatch,
        true,
        move |res| {
            match res {
                Ok(response) => f(Ok(response.data)),
                Err(err) => f(Err(err)),
            };
        },
    );
}

pub fn api_get_task<F>(id: i32, settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<Task, ApiError>) + 'static,
{
    make_request::<Task, (), _>(
        RequestMethod::Get,
        format!("tasks/{id}"),
        None,
        settings_dispatch,
        true,
        move |res| {
            match res {
                Ok(response) => f(Ok(response)),
                Err(err) => f(Err(err)),
            };
        },
    );
}

pub fn api_save_task<F>(task: Task, settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<Task, ApiError>) + 'static,
{
    let task_for_save = task.clone();

    let (method, path) = match task.id {
        Some(id) => (RequestMethod::Patch, format!("tasks/{id}")),
        None => (RequestMethod::Post, "tasks".to_owned()),
    };

    make_request::<Task, _, _>(
        method,
        path,
        Some(task_for_save),
        settings_dispatch,
        true,
        move |res| {
            match res {
                Ok(resp) => f(Ok(resp)),
                Err(err) => f(Err(err)),
            };
        },
    );
}

pub fn api_delete_task<F>(id: i32, settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<bool, ApiError>) + 'static,
{
    make_request::<bool, (), _>(
        RequestMethod::Delete,
        format!("tasks/{id}"),
        None,
        settings_dispatch,
        true,
        move |res| {
            match res {
                Ok(resp) => f(Ok(resp)),
                Err(err) => f(Err(err)),
            };
        },
    );
}

#[derive(Deserialize)]
struct TasksResponse {
    data: Vec<Task>,
}
