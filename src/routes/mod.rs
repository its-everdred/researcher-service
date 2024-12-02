mod data;
mod parse;
mod root;

use axum::Router;

pub fn all_routes() -> Router {
    Router::new()
        .merge(data::routes())
        .merge(root::routes())
        .merge(parse::routes())
}
