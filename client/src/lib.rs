use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

use crate::{composite::navbar::Navbar, routes::Route, store::settings_store::Settings};

#[cfg(feature = "real-api")]
mod api_real;

#[cfg(feature = "real-api")]
pub mod api {
    pub use crate::api_real::api_task as task;
    pub use crate::api_real::api_user as user;
}

#[cfg(feature = "test-api")]
mod api_test;

#[cfg(feature = "test-api")]
pub mod api {
    pub use crate::api_test::api_task_test as task;
    pub use crate::api_test::api_user_test as user;
}

pub mod common;
pub mod components;
pub mod composite;
pub mod model;
pub mod pages;
pub mod routes;
pub mod store;

#[component]
pub fn App() -> Html {
    let (_, dispatch) = use_store::<Settings>();
    use_effect_with((), move |_| {
        dispatch.reduce_mut(|settings| {
            settings.locale = "ru".to_owned();
        })
    });

    html! {
        <section class="section">
            <div class="is-paddingless">
                <BrowserRouter>
                    <Navbar />
                    <Switch<Route> render={routes::switch} />
                </BrowserRouter>
            </div>
        </section>

    }
}
