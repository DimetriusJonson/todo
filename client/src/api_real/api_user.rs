use serde::Deserialize;
use serde_json::json;
use yewdux::Dispatch;

use crate::{
    api_real::api_common::{RequestMethod, make_request}, common::api_error::ApiError, model::user::User, store::settings_store::Settings
};

pub fn api_login<F>(user: &User, settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<User, ApiError>) + 'static,
{
    let request_body = json!({
        "username": user.username,
        "password": user.password,
    });

    make_request::<LoginResponse, _, _>(
        RequestMethod::Post,
        "users/login".to_owned(),
        Some(request_body),
        settings_dispatch,
        false,
        move |res| {
            match res {
                Ok(resp) => f(Ok(User {
                    id: Some(resp.id),
                    username: resp.username.to_owned(),
                    password: None,
                    token: Some(resp.token),
                })),
                Err(err) => f(Err(err)),
            };
        },
    );
}

pub fn api_create_user<F>(user: User, settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<User, ApiError>) + 'static,
{
    let request_body = json!({
        "username": &user.username,
        "password": &user.password,
    });

    make_request::<CreateUserResponse, _, _>(
        RequestMethod::Post,
        "users".to_owned(),
        Some(request_body),
        settings_dispatch,
        false,
        move |res| match res {
            Ok(resp) => {
                let mut user = user.clone();
                user.id = Some(resp.id);
                user.password = None;
                f(Ok(user))
            }
            Err(err) => f(Err(err)),
        },
    );
}

pub fn api_logout<F>(settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<bool, ApiError>) + 'static,
{
    let dispatch_cloned = settings_dispatch.clone();
    make_request::<LogoutResponse, (), _>(
        RequestMethod::Get,
        "users/logout".to_owned(),
        None,
        settings_dispatch,
        true,
        move |res| {
            match res {
                Ok(response) => {
                    dispatch_cloned.reduce_mut(|settings| {
                        settings.user = None;
                        f(Ok(response.success))
                    });
                }
                Err(err) => f(Err(err)),
            };
        },
    );
}

#[derive(Deserialize)]
struct LoginResponse {
    id: u64,
    username: String,
    token: String,
}

#[derive(Deserialize)]
struct CreateUserResponse {
    id: u64,
}

#[derive(Deserialize)]
pub struct LogoutResponse {
    success: bool,
}
