use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::use_store;

use crate::{routes::Route, store::settings_store::Settings};

#[derive(PartialEq, Properties)]
pub struct Props {}

#[component]
pub fn Navbar(_props: &Props) -> Html {
    let (settings, _) = use_store::<Settings>();

    let nav_links_ref = use_node_ref();

    let burger_onclick = Callback::from({
        let nav_links_ref = nav_links_ref.clone();
        move |_: MouseEvent| {
            if let Some(nav_links) = nav_links_ref.cast::<HtmlElement>() {
                nav_links.class_list().toggle("is-active").unwrap();
            }
        }
    });

    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <Link<Route> classes="navbar-item is-size-3 has-text-weight-extrabold is-family-code mx-1" to={Route::Home}>{ "TODO" }</Link<Route>>

                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" onclick={burger_onclick}>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>

            <div ref={nav_links_ref} class="navbar-menu" id="nav-links">
                <div class="navbar-end">
                    <div class="buttons">
                        if let Some(user) = &settings.user && user.token.is_some() {
                            <div class="navbar-item"><Link<Route> classes="button is-warning is-light is-rounded" to={Route::Logout}>{format!("Выйти {}", user.username)}</Link<Route>></div>
                        } else {
                            <div class="navbar-item px-0"><Link<Route> classes="button is-warning is-soft is-rounded" to={Route::CreateUser}>{"Создать пользователя"}</Link<Route>></div>
                            <div class="navbar-item pl-0"><Link<Route> classes="button is-light is-rounded" to={Route::Login}>{"Войти"}</Link<Route>></div>
                        }
                    </div>
                </div>
            </div>

        </nav>
    }
}
