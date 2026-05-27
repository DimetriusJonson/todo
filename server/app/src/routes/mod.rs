use axum::{
    Router,
    extract::FromRef,
    middleware::{self},
    routing::{delete, get, patch, post},
};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

use crate::routes::check_middleware::{check_auth_token, check_json_accept};

pub mod check_middleware;
pub mod tasks_route;
pub mod users_route;

#[derive(Clone, FromRef)]
struct AppState {
    db_conn: DatabaseConnection,
}

pub async fn create_routes(db_conn: DatabaseConnection) -> Router {
    let app_state = AppState { db_conn };

    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/tasks", get(tasks_route::tasks))
                .route("/tasks/{id}", get(tasks_route::task))
                .route("/tasks", post(tasks_route::create_task))
                .route("/tasks/{id}", patch(tasks_route::update_task))
                .route("/tasks/{id}", delete(tasks_route::delete_task))
                .route("/users/logout", get(users_route::logout))
                .route_layer(middleware::from_fn_with_state(app_state.clone(), check_auth_token))
                .route("/health_check", get(users_route::health_check))
                .route("/users/login", post(users_route::login))
                .route("/users", post(users_route::create_user))
                .route("/users", get(users_route::users))
                .route_layer(middleware::from_fn(check_json_accept)),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
