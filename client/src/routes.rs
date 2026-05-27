use yew::{Html, html};
use yew_router::prelude::*;

use crate::pages::{
    create_user_page::CreateUserPage, home_page::HomePage, login_page::LoginPage,
    logout_page::LogoutPage, task_edit_page::TaskEditPage,
    task_page::TaskPage,
};

#[derive(PartialEq, Clone, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/createUser")]
    CreateUser,
    #[at("/logout")]
    Logout,
    #[at("/task/:id")]
    Task { id: i32 },
    #[at("/task/:id/edit")]
    TaskEdit { id: i32 },
    #[at("/task/create")]
    TaskCreate,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Login => html! {<LoginPage />},
        Route::Logout => html! {<LogoutPage />},
        Route::CreateUser => html! {<CreateUserPage />},
        Route::Task { id } => html! { <TaskPage id={id}/> },
        Route::TaskEdit { id } => html! { <TaskEditPage id={id} title="Редактировать задачу" /> },
        Route::TaskCreate => html! { <TaskEditPage title="Создать задачу" /> },
    }
}
