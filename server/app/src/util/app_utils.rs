use axum::http::{HeaderMap, HeaderValue, header::ACCEPT};
use mime::Mime;

use crate::util::app_error::AppResult;

pub fn is_accept_allow(headers: &HeaderMap<HeaderValue>) -> bool {
    let accept = headers.get(ACCEPT).iter().find_map(|hv| hv.to_str().ok());
    if let Some(accept) = accept {
        accept
            .parse::<mime::Mime>()
            .ok()
            .is_some_and(is_mime_compatible_json)
    } else {
        true
    }
}

fn is_mime_compatible_json(mime: Mime) -> bool {
    (mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().is_some_and(|name| name == "json")))
        || (mime.type_() == "*" && mime.subtype() == "*")
}

pub fn crypt_hash(password: String) -> AppResult<String> {
    Ok(bcrypt::hash(password, 12)?)
}

pub fn crypt_verify(password: String, hash: &str) -> AppResult<bool> {
    Ok(bcrypt::verify(password, hash)?)
}