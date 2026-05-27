use crate::database::tasks::{self, Entity as Tasks};
use crate::database::users::{self};
use crate::dto::task_dto::{self, TaskDto};
use crate::util::app_error::{AppError, AppResult};
use crate::util::app_json::ValidJson;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, IntoActiveModel, QueryFilter, TryIntoModel};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct TasksResponse {
    data: Vec<TaskDto>,
}

pub async fn tasks(
    State(db_conn): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
) -> AppResult<Json<TasksResponse>> {
    let tasks: Vec<TaskDto> = Tasks::find()
        .filter(tasks::Column::UserId.eq(user.id))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&db_conn)
        .await?
        .into_iter()
        .map(build_task)
        .collect();

    Ok(Json(TasksResponse { data: tasks }))
}

pub async fn task(
    Path(id): Path<i32>,
    Extension(user): Extension<users::Model>,
    State(db_conn): State<DatabaseConnection>,
) -> AppResult<Json<TaskDto>> {
    if let Some(task) = Tasks::find()
        .filter(tasks::Column::Id.eq(id))
        .filter(tasks::Column::UserId.eq(user.id))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&db_conn)
        .await?
    {
        Ok(Json(build_task(task)))
    } else {
        Err(AppError::not_found(format!("Not found task id={}", id)))
    }
}

pub async fn create_task(
    State(db_conn): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
    ValidJson(task): ValidJson<CreateTaskParams>,
) -> AppResult<Json<TaskDto>> {
    let saved_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        is_default: Set(task.is_default),
        completed_at: Set(task.completed_at),
        ..Default::default()
    }
    .save(&db_conn)
    .await?;

    Ok(Json(build_task(saved_task.try_into_model()?)))
}

pub async fn update_task(
    Path(id): Path<i32>,
    Extension(user): Extension<users::Model>,
    State(db_conn): State<DatabaseConnection>,
    ValidJson(task_dto): ValidJson<TaskDto>,
) -> AppResult<Json<TaskDto>> {
    if let Some(task) = Tasks::find()
        .filter(tasks::Column::Id.eq(id))
        .filter(tasks::Column::UserId.eq(user.id))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&db_conn)
        .await?
    {
        let mut active_task = task.into_active_model();

        if let Some(title) = task_dto.title {
            active_task.title.set_if_not_equals(title);
        }
        if let Some(description) = task_dto.description {
            active_task.description.set_if_not_equals(Some(description));
        }
        if let Some(priority) = task_dto.priority {
            active_task.priority.set_if_not_equals(Some(priority));
        }

        if let Some(completed_at) = task_dto.completed_at {
            if completed_at != DateTime::<FixedOffset>::MIN_UTC {
                active_task
                    .completed_at
                    .set_if_not_equals(task_dto.completed_at);
            } else {
                active_task.completed_at.set_if_not_equals(None);
            }
        }

        if !active_task.is_changed() {
            return Err(AppError::illegal_arguments("Нечего менять!"));
        }

        let active_task = active_task.update(&db_conn).await?;

        Ok(Json(build_task(active_task.try_into_model()?)))
    } else {
        Err(AppError::not_found(format!(
            "Not found task id={}",
            id
        )))
    }
}

pub async fn delete_task(
    Path(id): Path<i32>,
    Extension(user): Extension<users::Model>,
    State(db_conn): State<DatabaseConnection>,
) -> AppResult<Json<bool>> {
    if let Some(task) = Tasks::find()
        .filter(tasks::Column::Id.eq(id))
        .filter(tasks::Column::UserId.eq(user.id))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&db_conn)
        .await?
    {
        let mut active_task = task.into_active_model();
        active_task.deleted_at = Set(Some(Utc::now().fixed_offset()));
        let _ = active_task.save(&db_conn).await?;

        Ok(Json(true))
    } else {
        Err(AppError::not_found(format!(
            "Not found task id={}",
            id
        )))
    }
}

fn build_task(task: tasks::Model) -> TaskDto {
    TaskDto {
        id: Some(task.id),
        priority: task.priority,
        title: Some(task.title),
        description: task.description,
        is_default: task.is_default,
        completed_at: task.completed_at,
        deleted_at: task.deleted_at,
    }
}

#[derive(Serialize, Deserialize, Default, Validate, Debug)]
pub struct CreateTaskParams {
    pub priority: Option<String>,
    #[validate(required, regex(path = task_dto::title_regex(), message="Разрешены только буквы и цифры и не менее 3-х символов."))]
    pub title: Option<String>,
    pub completed_at: Option<DateTime<FixedOffset>>,
    pub description: Option<String>,
    pub is_default: Option<bool>,
}
