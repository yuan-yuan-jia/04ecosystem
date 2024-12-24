use std::sync::Arc;
use anyhow::Result;
use axum::{Json, Router, ServiceExt};
use axum::extract::State;
use axum::routing::{get, patch};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::{info, instrument};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, Layer};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[derive(Debug, Clone, PartialEq, Serialize)]
struct User {
    name: String,
    age: u8,
    skills: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UserUpdate {
    age: Option<u8>,
    skills: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);

    tracing_subscriber::registry().with(console).init();

    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };

    let user = Arc::new(Mutex::new(user));

    let addr = "0.0.0.0:8081";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);

    let app = Router::new()
        .route("/",get(user_handler))
        .route("/", patch(update_handler))
        .with_state(user);

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument]
async fn user_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    let t = user.lock().await;
    t.clone().into()
}

#[instrument]
async fn update_handler(
    State(user): State<Arc<Mutex<User>>>,
    Json(user_update): Json<UserUpdate>,
) -> Json<User> {
    let mut t =  user.lock().await;
    if let Some(age) = user_update.age {
        t.age = age;
    }

    if let Some(skills) = user_update.skills {
        t.skills = skills;
    }

    t.clone().into()
}