use ollama_rs::{
    generation::{
        completion::request::GenerationRequest,
        parameters::{FormatType, JsonSchema, JsonStructure},
    },
    Ollama,
};
use serde::Deserialize;
use crate::search::SearchResult;

#[derive(Debug, thiserror::Error)]
pub enum OllamaError {
    #[error("Ollama API error: {0}")]
    ApiError(#[from] ollama_rs::error::OllamaError),
    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

pub async fn query_ollama(
    prompt: &str,
    model: &str,
    format: Option<FormatType>,
) -> Result<String, OllamaError> {
    let ollama = Ollama::default();

    let mut gen_req =
        GenerationRequest::new(model.to_string(), prompt.to_string());

    if let Some(format_type) = format {
        gen_req = gen_req.format(format_type);
    }

    let res = ollama.generate(gen_req).await?;

    Ok(res.response)
}

pub async fn filter_search_results(
    sub_question: &str,
    results: &[SearchResult],
    model: &str,
) -> Result<Vec<SearchResult>, OllamaError> {
    if results.is_empty() {
        return Ok(Vec::new());
    }
    let results_str = results
        .iter()
        .map(|r| format!("Title: {} URL: {}", r.title, r.url))
        .collect::<Vec<String>>()
        .join("\n");
    let json_example = r#"{
  "results": [
    {
      "title": "Example Title",
      "url": "https://example.com"
    }
  ]
}"#;
    let prompt = format!("You are a search result filter. Your task is to identify relevant search results for a given topic. Below is the topic and a list of search results. Each search result is formatted as 'Title: [title] URL: [url]'.\n\nTopic: '{}'\n\nSearch Results:\n{}\n\nRespond with a JSON object containing a key 'results' which is an array of objects, where each object has 'title' and 'url' keys.\nExample:\n{}", sub_question, results_str, json_example);

    #[derive(JsonSchema, Deserialize, serde::Serialize)]
    struct FilteredResults {
        results: Vec<SearchResult>,
    }

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<FilteredResults>()));
    let response = query_ollama(&prompt, model, Some(format)).await?;

    let filtered_results: FilteredResults = serde_json::from_str(&response)?;
    Ok(filtered_results.results)
}

pub async fn summarize_text(
    text: &str,
    sub_question: &str,
    model: &str,
) -> Result<String, OllamaError> {
    let prompt = format!(
        "Summarize the following text in relation to the question: '{}'.\n\nText:\n{}",
        sub_question, text
    );

    #[derive(JsonSchema, Deserialize, serde::Serialize)]
    struct SummaryOutput {
        summary: String,
    }

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<SummaryOutput>()));
    let response = query_ollama(&prompt, model, Some(format)).await?;

    let summary_output: SummaryOutput = serde_json::from_str(&response)?;
    Ok(summary_output.summary)
}

pub async fn evaluate_completeness_and_answer(
    main_question: &str,
    global_summary: &str,
    model: &str,
) -> Result<String, OllamaError> {
    let prompt = format!("Synthesize the information in the research summary to directly answer the following question: '{}'.\n\nResearch Summary:\n{}", main_question, global_summary);

    #[derive(JsonSchema, Deserialize, serde::Serialize)]
    struct AnswerOutput {
        answer: String,
        #[serde(default)]
        refined_sub_questions: Vec<String>,
    }

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<AnswerOutput>()));
    let response = query_ollama(&prompt, model, Some(format)).await?;

    let answer_output: AnswerOutput = serde_json::from_str(&response)?;
    if answer_output.answer.is_empty() && !answer_output.refined_sub_questions.is_empty() {
        Ok(serde_json::to_string(&answer_output)?)
    } else {
        Ok(answer_output.answer)
    }
}

pub async fn decide_search_tool(
    sub_question: &str,
    model: &str,
) -> Result<String, OllamaError> {
    let prompt = format!("Given the sub-question: '{}', should I use Wikipedia or DuckDuckGo to find the answer? Respond with 'wikipedia' or 'duckduckgo'.", sub_question);

    #[derive(JsonSchema, Deserialize, serde::Serialize)]
    struct SearchToolDecision {
        tool: String,
    }

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<SearchToolDecision>()));
    let response = query_ollama(&prompt, model, Some(format)).await?;

    let decision: SearchToolDecision = serde_json::from_str(&response)?;
    Ok(decision.tool.trim().to_lowercase())
}

pub async fn list_ollama_models() -> Result<Vec<String>, OllamaError> {
    let ollama = Ollama::default();
    let models = ollama.list_local_models().await?;
    Ok(models.into_iter().map(|model| model.name).collect())
}