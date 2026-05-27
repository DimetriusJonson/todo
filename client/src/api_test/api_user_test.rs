use gloo::timers::callback::Timeout;
use yewdux::Dispatch;

use crate::{common::api_error::ApiError, model::user::User, store::settings_store::Settings};

pub fn api_login<F>(user: &User, _settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<User, ApiError>) + 'static,
{
    let user = user.clone();
    Timeout::new(1000, move || {
        if user.username == "aaa" && user.password.clone().unwrap_or_default() == "1234" {
            let mut user = user;
            user.token = Some("12345".to_owned());
            user.password = None;
            f(Ok(user))
        } else {
            f(Err(ApiError::other("Wrong username or password")))
        }
    }).forget();
}

pub fn api_create_user<F>(user: User, _settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<User, ApiError>) + 'static,
{
    let user = user.clone();
    Timeout::new(1000, move || {
        if user.username == "aaa" {
            f(Err(ApiError::other("User already exists!")))
        } else {
            let mut user = user;
            user.id = Some(1);
            f(Ok(user))
        }
    })
    .forget();
}

pub fn api_logout<F>(settings_dispatch: Dispatch<Settings>, f: F)
where
    F: Fn(Result<bool, ApiError>) + 'static,
{
    settings_dispatch.reduce_mut(|settings| settings.user = None);
    f(Ok(true))
}
