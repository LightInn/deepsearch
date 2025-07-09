use ollama_rs::Ollama;
use serde::Deserialize;
use crate::search::SearchResult;

#[derive(Debug, thiserror::Error)]
pub enum OllamaError {
    #[error("Ollama API error: {0}")]
    ApiError(#[from] ollama_rs::error::OllamaError),
    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

pub async fn query_ollama(prompt: &str, model: &str) -> Result<String, OllamaError> {
    let ollama = Ollama::default();

    let gen_req = ollama_rs::generation::completion::request::GenerationRequest::new(model.to_string(), prompt.to_string());
    let res = ollama.generate(gen_req).await?;

    Ok(res.response)
}

pub async fn filter_search_results(sub_question: &str, results: &[SearchResult], model: &str) -> Result<Vec<SearchResult>, OllamaError> {
    if results.is_empty() {
        return Ok(Vec::new());
    }
    let results_str = results.iter().map(|r| format!("Title: {} URL: {}", r.title, r.url)).collect::<Vec<String>>().join("\n");
    let json_example = r#"{
  "results": [
    {
      "title": "Example Title",
      "url": "https://example.com"
    }
  ]
}"#;
    let prompt = format!("You are a search result filter. Your task is to identify relevant search results for a given topic. Below is the topic and a list of search results. Each search result is formatted as 'Title: [title] URL: [url]'.\n\nTopic: '{}'\n\nSearch Results:\n{}\n\nRespond with a JSON object containing a key 'results' which is an array of objects, where each object has 'title' and 'url' keys.\nExample:\n{}", sub_question, results_str, json_example);
    let response = query_ollama(&prompt, model).await?;
    
    #[derive(Deserialize)]
    struct FilteredResults {
        results: Vec<SearchResult>,
    }

    let filtered_results: FilteredResults = serde_json::from_str(&response)?;
    Ok(filtered_results.results)
}

pub async fn summarize_text(text: &str, sub_question: &str, model: &str) -> Result<String, OllamaError> {
    let prompt = format!("Summarize the following text in relation to the question: '{}'.\n\nText:\n{}", sub_question, text);
    let response = query_ollama(&prompt, model).await?;
    Ok(response)
}

pub async fn evaluate_completeness_and_answer(main_question: &str, global_summary: &str, model: &str) -> Result<String, OllamaError> {
    let prompt = format!("Synthesize the information in the research summary to directly answer the following question: '{}'.\n\nResearch Summary:\n{}", main_question, global_summary);
    let response = query_ollama(&prompt, model).await?;
    Ok(response)
}

pub async fn decide_search_tool(sub_question: &str, model: &str) -> Result<String, OllamaError> {
    let prompt = format!("Given the sub-question: '{}', should I use Wikipedia or DuckDuckGo to find the answer? Respond with 'wikipedia' or 'duckduckgo'.", sub_question);
    let response = query_ollama(&prompt, model).await?;
    Ok(response.trim().to_lowercase())
}

pub async fn list_ollama_models() -> Result<Vec<String>, OllamaError> {
    let ollama = Ollama::default();
    let models = ollama.list_local_models().await?;
    Ok(models.into_iter().map(|model| model.name).collect())
}