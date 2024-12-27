use axum::{Json, Router, ServiceExt};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use http::{HeaderMap, StatusCode};
use http::header::LOCATION;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::Layer  as _;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Deserialize)]
struct ShortenReq {
    url: String,
}

#[derive(Debug, Serialize)]
struct ShortenRes {
    url: String,
}

#[derive(Debug, Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug, FromRow)]
struct UrlRecord {
    #[sqlx(default)]
    id: String,
    #[sqlx(default)]
    url: String,
}

const LISTEN_ADDR: &str = "0.0.0.0:9876";


impl AppState {
    async fn try_new(url: &str) -> anyhow::Result<Self> {
        let pool = PgPool::connect(url).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS urls (
                id CHAR(6) PRIMARY KEY,
                url TEXT NOT NULL UNIQUE
            )
            "#,
        ).execute(&pool).await?;

        Ok(Self { db: pool })
    }

    async fn shorten(&self, url: &str) -> anyhow::Result<String> {
        let id = nanoid::nanoid!(6);
        let ret = sqlx::query_as::<_, UrlRecord>(
            "INSERT INTO urls (id, url) VALUES ($1, $2) ON CONFLICT(url) DO UPDATE SET url=EXCLUDED.url RETURNING id",
        ).bind(&id)
            .bind(url)
            .fetch_one(&self.db)
            .await?;

        Ok(ret.id)
    }

    async fn get_url(&self, id: &str) -> anyhow::Result<String> {
        let ret =  sqlx::query_as::<_, UrlRecord>("SELECT url FROM urls WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        Ok(ret.url)
    }
}




#[tokio::main]
async  fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let url = "postgres://myuser:mypassword@localhost:5432/mydatabase";
    let state = AppState::try_new(url).await?;
    info!("Connected to database: {}", url);
    let listener = TcpListener::bind(LISTEN_ADDR).await?;
    info!("Listening on {}", LISTEN_ADDR);

    let app = Router::new()
        .route("/", post(shorten))
        .route("/:id", get(redirect))
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn shorten(
    State(state): State<AppState>,
    Json(data): Json<ShortenReq>,
) -> Result<impl IntoResponse, StatusCode> {

    let id = state.shorten(&data.url)
    .await.map_err(|err| {
        warn!("Failed to shorten URL: {}", err);
        StatusCode::UNPROCESSABLE_ENTITY
    })?;

    let body = Json(ShortenRes {
        url: format!("http://{}/{}", LISTEN_ADDR, id),
    });
    Ok((StatusCode::CREATED, body))
}

async fn redirect(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {

    let url = state.get_url(&id)
        .await.map_err(|_| {
        StatusCode::NOT_FOUND
    })?;

    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, url.parse().unwrap());
    Ok((StatusCode::PERMANENT_REDIRECT, headers))
}

