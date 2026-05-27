use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::use_store;

use crate::components::button::Button;
use crate::components::select_input::{SelectInput, SelectOption, SelectOptions};
use crate::composite::message_banner::MessageBanner;
use crate::composite::tasks_panel::TasksPanel;
use crate::routes::Route;
use crate::store::settings_store::{Filter, Settings, SortKind};

#[component]
pub fn HomePage() -> Html {
    let (settings, settings_dispatch) = use_store::<Settings>();
    let navigator = use_navigator().unwrap();
    let create_onclick = Callback::from(move |_: MouseEvent| {
        navigator.push(&Route::TaskCreate);
    });

    let filter_onchange = Callback::from({
        let settings_dispatch = settings_dispatch.clone();
        move |value: String| {
            let selected_filter = value.as_str().into();
            settings_dispatch.reduce_mut(move |settings| settings.filter = selected_filter);
        }
    });

    let sort_onchange = Callback::from(move |value: String| {
        let selected_sort = value.as_str().into();
        settings_dispatch.reduce_mut(move |settings| settings.sort_kind = selected_sort);
    });

    html! {
        <section class="section is-paddingless">
            <div class="container">
                <nav class="level">
                    <div class="level-left">
                        <div class="level-item">
                            <SelectInput name="filterSelect" not_selected_text={"Фильтр"} value={settings.filter.value()} options={filter_options()} onchange={filter_onchange}/>
                            <SelectInput class="ml-3" name="sortSelect" not_selected_text={"Сортировка"} value={settings.sort_kind.value()} options={sort_options()} onchange={sort_onchange}/>
                        </div>
                    </div>
                    <div class="level-right">
                        <Button class="level-item is-light" id="create_button" label="Создать" onclick={create_onclick}/>
                    </div>
                </nav>

                <TasksPanel/>
            </div>
            <MessageBanner/>
        </section>
    }
}

fn filter_options() -> SelectOptions {
    SelectOptions::new(vec![
        filter_to_option(Filter::Completed),
        filter_to_option(Filter::Uncompleted),
    ])
}

fn filter_to_option(filter: Filter) -> SelectOption {
    (filter.value(), filter.to_string())
}

fn sort_options() -> SelectOptions {
    SelectOptions::new(vec![
        sort_to_option(SortKind::Title),
        sort_to_option(SortKind::Priority),
    ])
}

fn sort_to_option(sort_kind: SortKind) -> SelectOption {
    (sort_kind.value(), sort_kind.to_string())
}
