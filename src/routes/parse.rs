use axum::{routing::get, Router};
use crate::controllers::parse;

pub fn routes() -> Router {
    Router::new()
        .route("/parse/eip/:id", get(parse::eip))
}
