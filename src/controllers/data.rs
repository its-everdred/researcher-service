use axum::{
    extract::Path,
    response::IntoResponse,
    http::StatusCode,
};

use crate::services::data::eip_data;

pub async fn eip(
    Path(id): Path<String>
) -> impl IntoResponse {
    // Validate id is a number
    let eip_id = match id.parse::<u32>() {
        Ok(num) => num,
        Err(_) => return (StatusCode::BAD_REQUEST, "EIP ID must be a number").into_response(),
    };

    let result = eip_data(eip_id);

    (StatusCode::OK, result).into_response()
}
