use axum::Json;
use serde::{Deserialize, Serialize};
use crate::services::scrape::ScrapeService;

#[derive(Deserialize)]
pub struct ScrapeRequest {
    url: String,
}

#[derive(Serialize)]
pub struct ScrapeResponse {
    data: String,  // Adjust this based on what your scraper service returns
}

pub async fn handler(
    Json(payload): Json<ScrapeRequest>,
) -> Json<ScrapeResponse> {
    let scrape_service = ScrapeService;
    let result = match scrape_service.scrape_url(&payload.url).await {
        Ok(content) => content,
        Err(_) => "Failed to scrape content".to_string(),
    };

    Json(ScrapeResponse { data: result })
}