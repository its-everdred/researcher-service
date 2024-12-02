use axum::{routing::get, Router};
use crate::controllers::data;

pub fn routes() -> Router {
    Router::new()
        .route("/data/eip/:id", get(data::eip))
}
