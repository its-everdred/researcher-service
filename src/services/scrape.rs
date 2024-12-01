use reqwest;
use scraper::{Html, Selector};

pub struct ScrapeService;

impl ScrapeService {
    pub async fn scrape_url(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        let document = Html::parse_document(&body);

        // Example: select content using CSS selectors
        let selector = Selector::parse("h1").unwrap();
        let title = document.select(&selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        Ok(title)
    }
}