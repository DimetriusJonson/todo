use axum::extract::State;
use axum::{Extension, Json};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Condition, DatabaseConnection, IntoActiveModel};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::database::users::{self, Entity as Users};
use crate::util::app_error::{AppError, AppResult};
use crate::util::app_json::ValidJson;
use crate::util::app_utils::{crypt_hash, crypt_verify};
use crate::util::jwt::create_token;

use sea_orm::EntityTrait;
use sea_orm::{ColumnTrait, QueryFilter};

#[derive(Deserialize, Validate)]
pub struct LoginParams {
    #[validate(required, length(min = 3))]
    username: Option<String>,
    #[validate(required, length(min = 4))]
    password: Option<String>,
}

#[derive(Serialize)]
pub struct LoginResponse {
    id: i32,
    username: String,
    token: String,
}

#[derive(Serialize)]
pub struct LogoutResponse {
    success: bool,
}

#[derive(Deserialize, Validate)]
pub struct CreateUserParams {
    #[validate(required, length(min = 3))]
    username: Option<String>,
    #[validate(required, length(min = 4))]
    password: Option<String>,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    id: i32,
    username: String,
}

#[derive(Serialize)]
pub struct UserDto {
    id: i32,
    name: String,
}

#[derive(Serialize)]
pub struct UsersResponse {
    data: Vec<UserDto>,
}

pub async fn users(State(db_conn): State<DatabaseConnection>) -> AppResult<Json<UsersResponse>> {
    let users = Users::find().all(&db_conn).await?;
    Ok(Json(UsersResponse {data: users.into_iter().map(build_user).collect()}))
}

fn build_user(user: users::Model) -> UserDto {
    UserDto {
        id: user.id,
        name: user.username
    }
}

pub async fn health_check(State(db_conn): State<DatabaseConnection>) -> AppResult<()> {
    let users = Users::find().all(&db_conn).await?;
    assert!(!users.is_empty());
    Ok(())
}

pub async fn login(
    State(db_conn): State<DatabaseConnection>,
    ValidJson(params): ValidJson<LoginParams>,
) -> AppResult<Json<LoginResponse>> {
    let user = Users::find()
        .filter(users::Column::Username.eq(params.username))
        .one(&db_conn)
        .await?
        .ok_or(AppError::illegal_arguments("Wrong username or password!"))?;

    if crypt_verify(params.password.unwrap(), &user.password)? {
        let token = create_token(user.id, user.username.to_owned())?;

        let mut saved_user = user.clone().into_active_model();
        saved_user.token = Set(Some(token.to_owned()));
        saved_user.save(&db_conn).await?;

        Ok(Json(LoginResponse { id: user.id, username: user.username, token }))
    } else {
        Err(AppError::illegal_arguments("Wrong username or password!"))
    }
}

pub async fn logout(
    State(db_conn): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
) -> AppResult<Json<LogoutResponse>> {
    let user = Users::find()
        .filter(users::Column::Id.eq(user.id))
        .one(&db_conn)
        .await?
        .ok_or(AppError::not_found(format!(
            "Not found user {}!",
            user.id
        )))?;

    let mut saved_user = user.clone().into_active_model();
    saved_user.token = Set(None);
    saved_user.save(&db_conn).await?;

    Ok(Json(LogoutResponse { success: true }))
}

pub async fn create_user(
    State(db_conn): State<DatabaseConnection>,
    ValidJson(params): ValidJson<CreateUserParams>,
) -> AppResult<Json<CreateUserResponse>> {
    let users = Users::find()
        .filter(Condition::all().add(users::Column::Username.eq(params.username.to_owned())))
        .all(&db_conn)
        .await?;

    if users.is_empty() {
        let user = users::ActiveModel {
            username: Set(params.username.unwrap().to_owned()),
            password: Set(crypt_hash(params.password.unwrap().to_owned())?),
            ..Default::default()
        }
        .save(&db_conn)
        .await?;

        return Ok(Json(CreateUserResponse { id: user.id.unwrap(), username: user.username.unwrap() }));
    }

    Err(AppError::illegal_arguments("User already exist!"))
}

pub async fn find_user_by_token(
    db_conn: DatabaseConnection,
    token: &str,
) -> AppResult<Option<users::Model>> {
    let user = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(&db_conn)
        .await?;

    Ok(user)
}
