use std::hash::Hash;
use std::time::Duration;
use axum::{Router, ServiceExt};
use axum::routing::get;
use opentelemetry_sdk::trace::Tracer;
use tokio::net::TcpListener;
use tokio::time::{sleep, Instant};
use tracing::{debug, info, instrument, warn};
use tracing::log::LevelFilter;
use tracing_subscriber::{fmt, Layer};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[tokio::main]
async  fn main() -> anyhow::Result<()> {
    let file_appender = tracing_appender::rolling::daily("./logs", "ecosystem.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty();
        //.with_filter(LevelFilter::Debug);

    let file = fmt::Layer::new()
        .with_writer(non_blocking)
        .pretty();
        //.with_filter(LevelFilter::Info);

    tracing_subscriber::registry()
        .with(console)
        .with(file)
        .init();

    let addr = "0.0.0.0:8081";
    let listener = TcpListener::bind(addr).await?;
    let app = Router::new().route("/", get(index_handler));
    info!("Starting server on {}", addr);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument]
async fn index_handler() -> &'static str {
    debug!("index handler started");
    sleep(Duration::from_secs(5)).await;
    let ret = long_task().await;
    info!(http.status = 200, "index handler completed");
    ret
}

#[instrument]
async fn long_task() -> &'static str {
    let start = Instant::now();
    sleep(Duration::from_secs(10)).await;
    let elapsed = start.elapsed().as_millis() as u64;
    warn!(app.task_duration = elapsed, "task takes too long");

    "Hello World!"
}


// fn init_tracer() -> anyhow::Result<Tracer> {
//     let tracer = opentelemetry_otlp::new_pipeline()
//         .tracing();
//
//     Ok(tracer)
// }