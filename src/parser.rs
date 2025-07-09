use reqwest::Client;
use scraper::{Html, Selector};

pub async fn fetch_and_parse_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36")
        .build()?;
    let html_content = client.get(url).send().await?.text().await?;

    let document = Html::parse_document(&html_content);

    // Try to find common content containers first
    let selectors = [
        "article",
        "main",
        "#content",
        ".content",
        "div.main-content",
        "body", // Fallback to body if no specific content is found
    ];

    let mut text_content = String::new();
    for sel_str in &selectors {
        let selector = Selector::parse(sel_str).unwrap();
        for element in document.select(&selector) {
            text_content.push_str(&element.text().collect::<Vec<_>>().join(" "));
            text_content.push_str("\n");
        }
        if !text_content.is_empty() {
            break; // Found content, no need to try other selectors
        }
    }

    Ok(text_content)
}