use axum::{
    extract::Path,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::services::parser::parse_eip_chunks;

pub async fn eip(
    Path(id): Path<String>
) -> impl IntoResponse {
    // Validate id is a number
    let eip_id = match id.parse::<u32>() {
        Ok(num) => num,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "error": "EIP ID must be a number" }))),
    };

    let chunks = match parse_eip_chunks(eip_id) {
        Ok(chunks) => chunks,
        Err(e) => return (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))),
    };
    (StatusCode::OK, Json(json!({ "chunks": chunks })))
}
