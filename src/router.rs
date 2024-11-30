use axum::{
    routing::get,
    Router,
};
use crate::controllers::root;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root::handler))
}