use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio::io;
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::Layer as _;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Clone, Deserialize, Serialize)]
struct Config {
    upstream_addr: String,
    listen_addr: String,
}



fn resolve_config() -> Config {
    Config {
        upstream_addr: "0.0.0.0:8080".to_string(),
        listen_addr: "0.0.0.0:8081".to_string(),
    }
}


async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> Result<()> {
    let (mut client_reader, mut client_writer) = client.split();
    let (mut upstream_reader, mut upstream_writer) = upstream.split();
    let client_to_upstream = io::copy(&mut client_reader, &mut upstream_writer);
    let upstream_to_client = io::copy(&mut upstream_reader, &mut client_writer);
    match tokio::try_join!(client_to_upstream, upstream_to_client) {
        Ok((n,m)) => {
            info!("proxied {} bytes from client to upstream, {} bytes from upstream to client", n, m);
        },
        Err(e) => {warn!("error proxying: {:?}", e)}
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {

    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let config = resolve_config();
    let config = Arc::new(config);
    info!("Upstream is {}", config.upstream_addr);
    info!("Listening on {}", config.listen_addr);
    let listener = TcpListener::bind(&config.listen_addr).await?;
    loop {
        let (client, addr) = listener.accept().await?;
        info!("Accepted connection from {}", addr);
        let cloned_config = config.clone();

        tokio::spawn(async move {
            let upstream = TcpStream::connect(&cloned_config.upstream_addr).await?;

            // 将client代理到上游
            proxy(client, upstream).await?;
            Ok::<_, anyhow::Error>(())
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}