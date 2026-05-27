use crate::{
    app::{ProxyApp, RedirectApp},
};
use anyhow::Result;
use pingora::{
    listeners::tls::TlsSettings, prelude::http_proxy_service, server::configuration::ServerConf,
    services::listening::Service,
};
use resource_proxy_pingora::StaticFilesHandler;
use std::sync::Arc;

pub fn proxy_service_tls(
    server_conf: Arc<ServerConf>,
    listen_addr: String,
    host_configs: Vec<HostConfig>,
    cert_path: String, 
    key_path: String
) -> Result<impl pingora::services::Service> {
    let proxy_app = ProxyApp::new(host_configs);
    let mut service = http_proxy_service(&server_conf, proxy_app);

    let tls_settings = TlsSettings::intermediate(&cert_path, &key_path)?;
    service.add_tls_with_settings(&listen_addr, None, tls_settings);

    Ok(service)
}

#[derive(Clone, Debug, Default)]
pub struct HostConfig {
    pub upstream_addr: Option<String>,
    pub hostnames: Vec<String>,
    pub path: Option<String>,
    pub static_files_handler: Option<StaticFilesHandler>,
    pub cors_allow_all: bool,
}

pub fn proxy_service_plain(
    server_conf: Arc<ServerConf>,
    listen_addr: String,
    host_configs: Vec<HostConfig>,
) -> impl pingora::services::Service {
    let proxy_app = ProxyApp::new(host_configs.clone());
    let mut service = http_proxy_service(&server_conf, proxy_app);

    service.add_tcp(&listen_addr);

    service
}

pub fn new_http_redirect_app(listen_addr: &str) -> Service<RedirectApp> {
    let mut service = Service::new("Echo Service HTTP".to_string(), RedirectApp {});
    service.add_tcp(listen_addr);
    service
}
