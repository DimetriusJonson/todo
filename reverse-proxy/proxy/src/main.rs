use anyhow::Result;
use dotenvy::dotenv;
use log::debug;
use pingora::{prelude::*, services::ServiceWithDependents};
use resource_proxy_pingora::StaticFilesConf;
use std::{collections::{BTreeMap, HashMap}, env};

use crate::service::{HostConfig, new_http_redirect_app, proxy_service_plain, proxy_service_tls};

mod app;
mod service;

fn main() -> Result<()> {
    let environment = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    let env_file_name = format!(".env.{}", environment);
    debug!("environment={environment}, env_file_name={env_file_name}");

    dotenv().ok();
    dotenvy::from_filename_override(env_file_name).ok();

    let http_proxy_port = env::var("HTTP_PROXY_PORT")?;
    let https_proxy_port = env::var("HTTPS_PROXY_PORT")?;
    let cert_path = env::var("CERT_PATH")?;
    let key_path = env::var("KEY_PATH")?;
    let cors_allow_all = env::var("CORS_ALLOW_ALL")?.parse::<bool>().unwrap_or(false);

    let mut http_app_hostnames = BTreeMap::new();
    let mut ssl_app_hostnames = BTreeMap::new();
    let mut app_paths = BTreeMap::new();
    let mut app_addrs = BTreeMap::new();

    let mut ssl_static_host_names = BTreeMap::new();
    let mut static_confs = BTreeMap::new();
    for (name, value) in env::vars() {
        if name.starts_with("APP_HOST_NAME_") {
            let uscore_index = name[name.rfind('_').unwrap() + 1..].to_string();
            http_app_hostnames.insert(uscore_index.to_owned(), build_hostnames(&value, &http_proxy_port));
            ssl_app_hostnames.insert(uscore_index, build_hostnames(&value, &https_proxy_port));
        } else if name.starts_with("APP_PATH_") {
            let uscore_index = name[name.rfind('_').unwrap() + 1..].to_string();
            app_paths.insert(uscore_index, value);
        } else if name.starts_with("APP_ADDR_") {
            let uscore_index = name[name.rfind('_').unwrap() + 1..].to_string();
            app_addrs.insert(uscore_index, value);
        } else if name.starts_with("STATIC_HOST_NAME_") {
            let uscore_index = name[name.rfind('_').unwrap() + 1..].to_string();
            ssl_static_host_names.insert(uscore_index, build_hostnames(&value, &https_proxy_port));
        } else if name.starts_with("STATIC_ROOT_") {
            let uscore_index = name[name.rfind('_').unwrap() + 1..].to_string();
            static_confs.insert(
                uscore_index,
                StaticFilesConf {
                    root: Some(value.into()),
                    index_file: vec!["index.html".to_owned()].into(),
                    ..Default::default()
                },
            );
        }
    }

    pretty_env_logger::init_timed();

    let opt = Some(Opt::parse_args());
    let mut my_server = Server::new(opt)?;
    //    let mut my_server = Server::new(None)?;
    my_server.bootstrap();

    //    let upstreams = LoadBalancer::try_from_iter(["127.0.0.1:3000"]).unwrap();
    //   let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));
    //    lb.add_tcp("0.0.0.0:6188");
    //    my_server.add_service(lb);

    let ssl_listen_addr = format!("0.0.0.0:{https_proxy_port}");
    let http_listen_addr = format!("0.0.0.0:{}", http_proxy_port);

    println!(
        "listen(http={http_listen_addr}, 
        https={ssl_listen_addr}); 
        app(http={http_app_hostnames:?}, 
        https={ssl_app_hostnames:?}, 
        ssl_statics={:?}, 
        app={app_addrs:?}, 
        cors_allow_all={cors_allow_all})",
        &ssl_static_host_names
    );

    let mut ssl_host_configs = vec![];
    for (index, host_name) in ssl_app_hostnames.iter() {
        ssl_host_configs.push(HostConfig {
            upstream_addr: Some(app_addrs[index].to_owned()),
            hostnames: host_name.to_owned(),
            path: Some(app_paths[index].to_owned()),
            static_files_handler: None,
            cors_allow_all,
        });
    }

    for (index, host_name) in ssl_static_host_names.iter() {
        ssl_host_configs.push(HostConfig {
            upstream_addr: None,
            hostnames: host_name.to_owned(),
            static_files_handler: Some(static_confs[index].to_owned().try_into().unwrap()),
            ..Default::default()
        });
    }

    let proxy_service_ssl = proxy_service_tls(
        my_server.configuration.clone(),
        ssl_listen_addr.to_owned(),
        ssl_host_configs,
        cert_path.to_owned(),
        key_path.to_owned(),
    )?;

    let http_redirect_app = new_http_redirect_app(&http_listen_addr);

    let mut http_host_configs = vec![];
    for (index, host_name) in ssl_app_hostnames.iter() {
        http_host_configs.push(HostConfig {
            upstream_addr: Some(app_addrs[index].to_owned()),
            hostnames: host_name.to_owned(),
            path: Some(app_paths[index].to_owned()),
            static_files_handler: None,
            cors_allow_all,
        });
    }

    let proxy_service_plain = proxy_service_plain(
        my_server.configuration.clone(),
        http_listen_addr,
        http_host_configs,
    );

    let services: Vec<Box<dyn ServiceWithDependents>> = vec![
        Box::new(proxy_service_ssl),
        //        Box::new(http_redirect_app),
        Box::new(proxy_service_plain),
    ];
    my_server.add_services(services);

    debug!("run_forever...");
    my_server.run_forever();
}

fn build_hostnames(host_name: &str, port: &str) -> Vec<String> {
    let mut result = host_name.to_owned();
    result.retain(|c| !c.is_whitespace());

    result
        .split(',')
        .map(|h| {
            if port == "80" || port == "443" {
                h.to_owned()
            } else {
                format!("{}:{}", h, port)
            }
        })
        .collect()
}

/*

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    /// For this small example, we don't need context storage
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self
            .0
            .select(b"", 256) // hash doesn't matter for round robin
            .unwrap();

        println!("upstream peer is: {upstream:?}");

        // Set SNI to one.one.one.one
        let peer = Box::new(HttpPeer::new(upstream, false, "localhost:3000".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request.insert_header("Host", "localhost").unwrap();
        Ok(())
    }
}
*/
