use std::fmt::Display;

use serde::{Deserialize, Serialize};
use yewdux::Store;

use crate::model::user::User;

#[derive(Clone, PartialEq, Eq, Store, Default, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct Settings {
    pub locale: String,
    pub user: Option<User>,
    #[serde(skip)]
    pub api_in_progress: bool,
    pub filter: Filter,
    pub sort_kind: SortKind,
}

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Filter {
    #[default]
    Empty,
    Completed,
    Uncompleted,
}

impl Filter {
    pub fn value(&self) -> String {
        match self {
            Filter::Empty => "".to_owned(),
            Filter::Completed => "completed".to_owned(),
            Filter::Uncompleted => "uncompleted".to_owned(),
        }
    }
}

impl From<&str> for Filter {
    fn from<'a>(value: &str) -> Self {
        match value {
            "completed" => Filter::Completed,
            "uncompleted" => Filter::Uncompleted,
            _ => Filter::Empty,
        }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::Empty => write!(f, "Не выбран"),
            Filter::Completed => write!(f, "Завершенные"),
            Filter::Uncompleted => write!(f, "Незавершенные"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SortKind {
    #[default]
    Empty,
    Title,
    Priority,
}

impl SortKind {
    pub fn value(&self) -> String {
        match self {
            SortKind::Empty => "".to_owned(),
            SortKind::Title => "title".to_owned(),
            SortKind::Priority => "priority".to_owned(),
        }
    }
}

impl From<&str> for SortKind {
    fn from<'a>(value: &str) -> Self {
        match value {
            "title" => SortKind::Title,
            "priority" => SortKind::Priority,
            _ => SortKind::Empty,
        }
    }
}

impl Display for SortKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortKind::Empty => write!(f, "Не выбран"),
            SortKind::Title => write!(f, "Название"),
            SortKind::Priority => write!(f, "Приоритет"),
        }
    }
}
