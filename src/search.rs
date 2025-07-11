use serde::{Deserialize, Serialize};
use scraper::{Html, Selector};
use schemars::JsonSchema;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    query: Query,
}

#[derive(Debug, Deserialize)]
struct WikipediaSearchEntry {
    title: String,
}

#[derive(Debug, Deserialize)]
struct Query {
    search: Vec<WikipediaSearchEntry>,
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
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
            ("srprop", "titles"), // Request only title to construct URL
        ])
        .send()
        .await?;

    let api_response = response.json::<ApiResponse>().await?;

    Ok(api_response.query.search.into_iter().map(|sr| {
        SearchResult {
            title: sr.title.clone(),
            url: format!("https://en.wikipedia.org/wiki/{}", sr.title.replace(" ", "_")),
        }
    }).collect())
}

pub async fn search_duckduckgo(search_term: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://html.duckduckgo.com/html/?q={}", search_term);
    

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
