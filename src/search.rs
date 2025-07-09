use serde::Deserialize;
use scraper::{Html, Selector};

#[derive(Debug, Deserialize)]
struct ApiResponse {
    query: Query,
}

#[derive(Debug, Deserialize)]
struct Query {
    search: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}

pub async fn search_wikipedia(search_term: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "https://en.wikipedia.org/w/api.php";

    let response = client
        .get(url)
        .query(&[
            ("action", "query"),
            ("list", "search"),
            ("srsearch", search_term),
            ("format", "json"),
            ("srprop", "snippet|titles|url"), // Request URL as well
        ])
        .send()
        .await?;

    let api_response = response.json::<ApiResponse>().await?;

    Ok(api_response.query.search.into_iter().map(|mut sr| {
        sr.url = format!("https://en.wikipedia.org/wiki/{}", sr.title.replace(" ", "_"));
        sr
    }).collect())
}

pub async fn search_duckduckgo(search_term: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://html.duckduckgo.com/html/?q={}", search_term);
    println!("duck!");

    let html_content = client.get(&url).send().await?.text().await?;

    let document = Html::parse_document(&html_content);
    let selector = Selector::parse("a.result__a").unwrap();

    let results: Vec<SearchResult> = document
        .select(&selector)
        .filter_map(|element| {
            let title = element.inner_html();
            let url = element.value().attr("href").map(|s| {
                if s.starts_with("//") {
                    format!("https:{}", s)
                } else {
                    s.to_string()
                }
            });
            url.map(|u| SearchResult { title, url: u })
        })
        .collect();

    Ok(results)
}
