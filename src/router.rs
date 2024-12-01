use axum::Router;
use crate::routes;

pub fn create_router() -> Router {
    routes::all_routes()
}