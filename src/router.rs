use axum::{
    routing::{get, post},
    Router,
};
use crate::controllers::root;
use crate::controllers::scrape;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root::handler))
        .route("/scrape", post(scrape::handler))
}