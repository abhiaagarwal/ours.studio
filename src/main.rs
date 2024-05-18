use std::net::SocketAddr;

use askama_axum::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use color_eyre::Result;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Template)]
#[template(path = "index.html.jinja")]
struct IndexTemplate {
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ours-studio=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .nest_service("/assets", ServeDir::new("assets"));
    let listener = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Listening on {listener}");

    axum_server::bind(listener)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn health() -> impl IntoResponse {
    "healthy :)".into_response()
}

async fn index() -> IndexTemplate {
    IndexTemplate {
        name: "world".into(),
    }
}
