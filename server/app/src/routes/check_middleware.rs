use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use headers::{Authorization, HeaderMapExt, authorization::Bearer};
use sea_orm::DatabaseConnection;

use crate::{
    routes::users_route::find_user_by_token,
    util::{
        app_error::{AppError, AppResult},
        app_utils,
        jwt::is_valid_token,
    },
};

pub async fn check_json_accept(request: Request<Body>, next: Next) -> AppResult<Response> {
    if !app_utils::is_accept_allow(request.headers()) {
        return Err(AppError::illegal_media_type("Wrong Accept".to_owned()));
    }

    Ok(next.run(request).await)
}

pub async fn check_auth_token(
    State(db_conn): State<DatabaseConnection>,
    mut request: Request<Body>,
    next: Next,
) -> AppResult<Response> {
    let bearer = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(AppError::unauthorized("No authorization token."))?;

    let token = bearer.token();
    if is_valid_token(token)? {
        request.extensions_mut().insert(
            find_user_by_token(db_conn, token)
                .await?
                .ok_or(AppError::unauthorized("Not found user for authorization token."))?,
        );

        Ok(next.run(request).await)
    } else {
        Err(AppError::unauthorized("Invalid authorization token."))
    }
}
