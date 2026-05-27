use std::sync::OnceLock;

use chrono::{DateTime, FixedOffset};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Default, Validate, Debug)]
pub struct TaskDto {
    pub id: Option<i32>,
    pub priority: Option<String>,
    #[validate(regex(path = title_regex(), message="Разрешены только буквы и цифры и не менее 3-х символов."))]
    pub title: Option<String>,
    pub completed_at: Option<DateTime<FixedOffset>>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTime<FixedOffset>>,
    pub is_default: Option<bool>,
}

pub fn title_regex() -> &'static Regex {
    static RE_POSTAL_CODE: OnceLock<Regex> = OnceLock::new();
    RE_POSTAL_CODE.get_or_init(|| Regex::new("^[А-Яа-яA-Za-z0-9 ]{3,}$").unwrap())
}
