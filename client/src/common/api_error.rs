use std::{collections::BTreeMap, error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ApiError {
    UnAuthorized(String),
    Validation(String, BTreeMap<String, String>),
    Network(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnAuthorized(msg) => write!(f, "Пользователь не авторизован. {}", msg),
            Self::Network(msg) => write!(f, "Ошибка запроса: {}.", msg),
            Self::Validation(msg, _) => write!(f, "Неверные параметры. {}", msg),
        }
    }
}

impl Error for ApiError {}

impl From<gloo_net::Error> for ApiError {
    fn from(error: gloo_net::Error) -> Self {
        match error {
            gloo_net::Error::GlooError(msg) => ApiError::Network(msg),
            gloo_net::Error::JsError(js_error) => ApiError::Network(js_error.message),
            gloo_net::Error::SerdeError(error) => ApiError::Network(error.to_string()),
        }
    }
}
