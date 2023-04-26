use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use bonsaidb::local::config::{Builder, StorageConfiguration};
use bonsaidb::{core::schema::Collection, local::Database};
use serde::{Deserialize, Serialize};

const VERSION: &str = "Wayfarer Backend v0.1.0";

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "messages")]
struct Message {
    pub contents: String,
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        // initialize tracing
        tracing_subscriber::fmt::init();
    }
    let db = Database::open::<Message>(StorageConfiguration::new("basic.bonsaidb")).unwrap();

    // build our application with a route
    let app = Router::new()
        // Print version if no parameters are given
        .route("/", get(|| async { VERSION }))
        // Validate a given user name
        .route("/validate_user", post(validate_user));

    // run our app with hyper
    let addr = ([0, 0, 0, 0], 3000).into();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn validate_user(Json(payload): Json<String>) -> StatusCode {
    if payload == "Exists" {
        StatusCode::FOUND
    } else {
        StatusCode::NOT_FOUND
    }
}
