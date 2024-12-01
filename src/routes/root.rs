use axum::{routing::get, Router};
use crate::controllers::root;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root::handler))
}
