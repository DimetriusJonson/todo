use std::collections::BTreeMap;

use gloo::utils::window;
use gloo_net::http::{Request, RequestBuilder, Response};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use yew::platform::spawn_local;
use yewdux::Dispatch;

use crate::{common::api_error::ApiError, store::settings_store::Settings};

pub fn make_request<T, B, F>(
    method: RequestMethod,
    path: String,
    request_body: Option<B>,
    settings_dispatch: Dispatch<Settings>,
    secured: bool,
    f: F,
) where
    T: DeserializeOwned,
    B: Serialize + 'static,
    F: Fn(Result<T, ApiError>) + 'static,
{
    settings_dispatch.reduce_mut(|s| s.api_in_progress = true);
    spawn_local(async move {
        let url = format!("{}/api/v1/{path}", get_host_url());

        let req = match method {
            RequestMethod::Post => Request::post(&url),
            RequestMethod::Patch => Request::patch(&url),
            RequestMethod::Get => Request::get(&url),
            RequestMethod::Delete => Request::delete(&url),
        };

        let req = match request_body {
            Some(_) => req.header("Content-Type", "application/json"),
            None => req,
        };
        let req = req.header("Accept", "application/json");

        match prepare_request(&settings_dispatch, req, secured) {
            Ok(req) => {
                let response = match request_body {
                    Some(request_body) => req.json(&request_body).unwrap().send().await,
                    None => req.send().await,
                };
                let result = process_response(settings_dispatch.clone(), response).await;

                settings_dispatch.reduce_mut(|s| s.api_in_progress = false);
                f(result)
            }
            Err(err) => { 
                settings_dispatch.reduce_mut(|s| s.api_in_progress = false);
                f(Err(err)) 
            },
        };
    });
}

fn prepare_request(
    settings_dispatch: &Dispatch<Settings>,
    req: RequestBuilder,
    secured: bool,
) -> Result<RequestBuilder, ApiError> {
    let token = get_settings_token(settings_dispatch);

    if secured {
        if let Some(token) = token {
            Ok(req.header("Authorization", &format!("Bearer {}", token)))
        } else {
            Err(ApiError::UnAuthorized("No token!".to_owned()))?
        }
    } else {
        Ok(req)
    }
}

async fn process_response<T>(
    settings_dispatch: Dispatch<Settings>,
    response: Result<Response, gloo_net::Error>,
) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    match response {
        Ok(response) => {
            if response.status() == 200 {
                let data = response.json::<T>().await?;
                Ok(data)
            } else if response.status() == 401 {
                let error_resp = response.json::<ErrorResponse>().await?;
                settings_dispatch.reduce_mut(|settings| settings.user = None);

                Err(ApiError::UnAuthorized(error_resp.error))
            } else if response.status() == 422 {
                let error_resp = response.json::<ErrorResponse>().await?;
                let mut errors_map = BTreeMap::new();
                for field_error in error_resp.error.split('\n') {
                    if let Some(sep_index) = field_error.find(':') {
                        errors_map.insert(
                            field_error[..sep_index].to_owned(),
                            field_error[sep_index + 1..].to_owned(),
                        );
                    }
                }
                Err(ApiError::Validation(error_resp.error, errors_map))
            } else {
                let error_resp = response.json::<ErrorResponse>().await?;
                Err(ApiError::Network(error_resp.error))
            }
        }
        Err(error) => Err(ApiError::Network(error.to_string())),
    }
}

fn get_host_url() -> String {
    let location = window().location();
    format!(
        "{}//{}",
        location.protocol().unwrap_or("https".to_owned()),
        location.hostname().unwrap_or("localhost".to_owned())
    )
}

fn get_settings_token(settings_dispatch: &Dispatch<Settings>) -> Option<String> {
    let settings = settings_dispatch.get();
    if let Some(user) = &settings.user
        && let Some(token) = user.token.to_owned()
    {
        return Some(token);
    }
    None
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

pub enum RequestMethod {
    Post,
    Patch,
    Get,
    Delete,
}
