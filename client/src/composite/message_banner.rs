use std::fmt::Display;

use chrono::{DateTime, Local};
use gloo::timers::callback::Timeout;
use web_sys::HtmlButtonElement;
use yew::prelude::*;
use yewdux::{Dispatch, Store, use_store};

#[derive(Clone, PartialEq, Eq, Store, Default)]
enum MessageKind {
    #[default]
    None,
    Info(String, bool),
    Error(String),
}

impl MessageKind {
    fn style(&self) -> String {
        match self {
            MessageKind::Info(_, _) => "is-primary".to_owned(),
            MessageKind::Error(_) => "is-danger".to_owned(),
            MessageKind::None => "".to_owned(),
        }
    }
}

impl Display for MessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageKind::Info(msg, _) => write!(f, "{}", msg),
            MessageKind::Error(msg) => write!(f, "{}", msg),
            MessageKind::None => write!(f, ""),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Store, Default)]
struct Message {
    id: usize,
    kind: MessageKind,
    hidden: Option<DateTime<Local>>,
}

impl Message {
    fn new_error(id: usize, msg: String) -> Self {
        Self {
            id,
            kind: MessageKind::Error(msg),
            hidden: None,
        }
    }

    fn new_info(id: usize, msg: String) -> Self {
        Self {
            id,
            kind: MessageKind::Info(msg, false),
            hidden: None,
        }
    }

    fn _display_style(&self) -> String {
        if self.hidden.is_none() {
            "show-message".to_owned()
        } else {
            "hide-message".to_owned()
        }
    }

    fn is_live(&self) -> bool {
        if let Some(hide_time) = self.hidden {
            let delta = Local::now() - hide_time;
            return delta.as_seconds_f64() < 1.0;
        }

        true
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Clone, PartialEq, Eq, Store, Default)]
pub struct MessageStore {
    messages: Vec<Message>,
}

impl MessageStore {

    fn add_message(&mut self, msg: Message) -> usize {
        self.messages.retain(|m|m.is_live());

        let id = self.messages.iter().map(|m|m.id).max().unwrap_or_default() + 1;
        self.messages.push(Message{id, ..msg});
        id
    }

    fn add_info(&mut self, msg: String) -> usize {
        self.add_message(Message::new_info(0, msg))
    }

    fn add_error(&mut self, msg: String) -> usize {
        self.add_message(Message::new_error(0, msg))
    }

    fn remove(&mut self, id: usize) {
        if let Some(msg) = self.messages.iter_mut().find(|m| m.id == id) {
            msg.hidden = Some(Local::now());
        }
    }
}

fn set_timer(dispatch: &Dispatch<MessageStore>, id: usize, millis: u32) {
    let dispatch = dispatch.clone();
    Timeout::new(millis, move || {
        dispatch.reduce_mut(move |message_store| message_store.remove(id));
    })
    .forget();
}

pub fn show_error(dispatch: &Dispatch<MessageStore>, message: &str) {
    dispatch.reduce_mut(|message_state| {
        let id = message_state.add_error(message.to_owned());
        set_timer(dispatch, id, 30000);
    });
}

pub fn show_info(dispatch: &Dispatch<MessageStore>, message: &str) {
    dispatch.reduce_mut(|message_state| {
        let id = message_state.add_info(message.to_owned());
        set_timer(dispatch, id, 5000);
    });
}

#[derive(PartialEq, Properties)]
pub struct Props {}

#[component]
pub fn MessageBanner(_props: &Props) -> Html {
    let (message_store, dispatch) = use_store::<MessageStore>();

    let onclick = {
        let dispatch = dispatch.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target_unchecked_into::<HtmlButtonElement>();
            let id = target.id();
            let id = id[id.find('_').unwrap() + 1..].parse::<usize>().unwrap();
            dispatch.reduce_mut(move |message_store| message_store.remove(id));
        })
    };

    html! {
        <div class="has-text-centered py-3" style={"
            position: fixed;
            left: 0;
            bottom: 0;
            width: 100%;
            z-index: 1000;
        "}>
            for message in &message_store.messages.iter().filter(|m|m.hidden.is_none()).collect::<Vec<&Message>>() {
                <p class="field">
                    <span class={format!("tag is-medium {}", message.kind.style())}>
                        {&message.to_string()}
                        <button class="delete is-small" id={format!("m_{}", message.id)} onclick={onclick.clone()}></button>
                    </span>
                </p>
            }
        </div>
    }
}
