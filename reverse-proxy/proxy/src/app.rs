use async_trait::async_trait;
use http::{Method, Response, StatusCode, header};
use log::debug;
use pingora::{
    Error, ErrorType,
    apps::http_app::ServeHttp,
    http::{RequestHeader, ResponseHeader},
    prelude::{HttpPeer, ProxyHttp, Result, Session},
    protocols::http::ServerSession,
};
use resource_proxy_pingora::RequestFilter;

use crate::service::HostConfig;

pub struct ProxyApp {
    host_configs: Vec<HostConfig>,
}

impl ProxyApp {
    pub fn new(host_configs: Vec<HostConfig>) -> Self {
        Self { host_configs }
    }

    fn get_host_config(&self, session: &mut Session) -> Option<&HostConfig> {
        let path = String::from_utf8_lossy(session.req_header().raw_path());
        println!("path: {}", path);

        session
            .get_header(header::HOST)
            .iter()
            .filter_map(|hv| hv.to_str().ok())
            .find_map(|host| {
                self.host_configs.iter().find(|c| {
                    c.hostnames.contains(&host.to_owned())
                        && (c.path.is_none() || path.starts_with(&c.path.to_owned().unwrap()))
                })
            })
    }

    fn unchecked_session_host(session: &Session) -> String {
        session
            .get_header(header::HOST)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    }
}

#[async_trait]
impl ProxyHttp for ProxyApp {
    type CTX = ();
    fn new_ctx(&self) {}

    async fn proxy_upstream_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<bool> {
        println!("Host: {:?}", session.get_header(header::HOST));

        let mut allow = matches!(self.get_host_config(session), Some(host_config) if host_config.upstream_addr.is_some());
        if allow && session.req_header().method == Method::OPTIONS {
            allow = false;
        }

        Ok(allow)
    }

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        if let Some(host_config) = self.get_host_config(session)
            && let Some(app_addr) = &host_config.upstream_addr
        {
            return Ok(Box::new(HttpPeer::new(
                app_addr.as_str(),
                false,
                Self::unchecked_session_host(session),
            )));
        }

        Err(Error::explain(
            ErrorType::HTTPStatus(StatusCode::BAD_REQUEST.as_u16()),
            "Wrong Host",
        ))
    }

    async fn request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<bool, Box<Error>> {
        if let Some(host_config) = self.get_host_config(session) {
            if let Some(static_files_handler) = &host_config.static_files_handler {
                static_files_handler.handle(session, ctx).await?;
            } else {
                if host_config.cors_allow_all && session.req_header().method == Method::OPTIONS {
                    println!("Add Cors Allow All Headers");

                    let mut header = ResponseHeader::build(StatusCode::OK, Some(4))?;
                    header.append_header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")?;
                    header.append_header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")?;
                    header.append_header(header::ACCESS_CONTROL_MAX_AGE, "1728000")?;
                    header.append_header(
                        header::ACCESS_CONTROL_ALLOW_METHODS,
                        "GET,POST,PATCH,DELETE",
                    )?;
                    header.append_header(header::ACCESS_CONTROL_ALLOW_HEADERS, "*")?;
                    session
                        .write_response_header(Box::new(header), true)
                        .await?;
                    return Ok(true);
                }
            }
        }

        session.upstream_compression.adjust_decompression(true);
        session.upstream_compression.adjust_level(9);

        Ok(false)
    }

    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        if let Some(client_addr) = session.client_addr() {
            upstream_request.insert_header(header::FORWARDED, client_addr.to_string())?;
        }
        Ok(())
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        upstream_response.insert_header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")?;

        Ok(())
    }
}

pub struct RedirectApp;

#[async_trait]
impl ServeHttp for RedirectApp {
    async fn response(&self, http_stream: &mut ServerSession) -> Response<Vec<u8>> {
        if let Some(host_header) = http_stream.get_header(header::HOST)
            && let Ok(host) = host_header.to_str()
        {
            let body = "<html><body>301 Moved Permanently</body></html>"
                .as_bytes()
                .to_owned();
            return Response::builder()
                .status(StatusCode::MOVED_PERMANENTLY)
                .header(header::CONTENT_TYPE, "text/html")
                .header(header::CONTENT_LENGTH, body.len())
                .header(header::LOCATION, format!("https://{host}"))
                .body(body)
                .unwrap();
        }

        debug!(
            "Not found host_config for Host {:?}",
            http_stream.get_header(header::HOST)
        );
        let body = "<html><body>Not found</body></html>".as_bytes().to_owned();
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "text/html")
            .header(header::CONTENT_LENGTH, body.len())
            .body(body)
            .unwrap()
    }
}
