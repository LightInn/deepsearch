use ollama_rs::{
    generation::{
        completion::request::GenerationRequest,
        parameters::{FormatType, JsonSchema, JsonStructure},
    },
    Ollama,
};
use serde::Deserialize;
use crate::search::SearchResult;
use crate::prompts;

#[derive(Debug, thiserror::Error)]
pub enum OllamaError {
    #[error("Ollama API: {0}")]
    ApiError(#[from] ollama_rs::error::OllamaError),
    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

pub async fn query_ollama(
    prompt: &str,
    model: &str,
    format: Option<FormatType>,
    verbose: bool,
) -> Result<String, OllamaError> {
    let ollama = Ollama::default();

    let mut gen_req =
        GenerationRequest::new(model.to_string(), prompt.to_string());

    if let Some(format_type) = format {
        gen_req = gen_req.format(format_type);
    }

    let res = ollama.generate(gen_req).await?;

    if verbose {
        println!("Response: {}", &res.response);
    }

    Ok(res.response)
}

pub async fn filter_search_results(
    sub_question: &str,
    results: &[SearchResult],
    model: &str,
    verbose: bool,
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
    let prompt = prompts::filter_search_results_prompt(sub_question, &results_str, &json_example);

    #[derive(JsonSchema, Deserialize, serde::Serialize)]
    struct FilteredResults {
        results: Vec<SearchResult>,
    }

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<FilteredResults>()));
    let response = query_ollama(&prompt, model, Some(format), verbose).await?;

    let filtered_results: FilteredResults = serde_json::from_str(&response)?;
    Ok(filtered_results.results)
}

pub async fn summarize_text(
    text: &str,
    sub_question: &str,
    model: &str,
    verbose: bool,
) -> Result<String, OllamaError> {
    let prompt = prompts::summarize_text_prompt(sub_question, text);

    #[derive(JsonSchema, Deserialize, serde::Serialize)]
    struct SummaryOutput {
        summary: String,
    }

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<SummaryOutput>()));
    let response = query_ollama(&prompt, model, Some(format), verbose).await?;

    let summary_output: SummaryOutput = serde_json::from_str(&response)?;
    Ok(summary_output.summary)
}

pub async fn evaluate_completeness_and_answer(
    main_question: &str,
    global_summary: &str,
    model: &str,
    verbose: bool,
) -> Result<String, OllamaError> {
    let prompt = prompts::evaluate_completeness_and_answer_prompt(main_question, global_summary);

    #[derive(JsonSchema, Deserialize, serde::Serialize)]
    struct AnswerOutput {
        answer: String,
        #[serde(default)]
        refined_sub_questions: Vec<String>,
    }

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<AnswerOutput>()));
    let response = query_ollama(&prompt, model, Some(format), verbose).await?;

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
    verbose: bool,
) -> Result<String, OllamaError> {
    let prompt = prompts::decide_search_tool_prompt(sub_question);

    #[derive(JsonSchema, Deserialize, serde::Serialize)]
    struct SearchToolDecision {
        tool: String,
    }

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<SearchToolDecision>()));
    let response = query_ollama(&prompt, model, Some(format), verbose).await?;

    let decision: SearchToolDecision = serde_json::from_str(&response)?;
    if verbose {
        println!("AI decided to use: {}", decision.tool);
    }
    Ok(decision.tool.trim().to_lowercase())
}

pub async fn list_ollama_models() -> Result<Vec<String>, OllamaError> {
    let ollama = Ollama::default();
    let models = ollama.list_local_models().await?;
    Ok(models.into_iter().map(|model| model.name).collect())
}

pub async fn decompose_question(
    question: &str,
    model: &str,
    verbose: bool,
) -> Result<Vec<String>, OllamaError> {
    let prompt = prompts::decompose_question_prompt_initial(question);

    #[derive(JsonSchema, Deserialize, serde::Serialize)]
    struct DecomposedQuestion {
        sub_questions: Vec<String>,
    }

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<DecomposedQuestion>()));
    let response = query_ollama(&prompt, model, Some(format), verbose).await?;

    let decomposed: DecomposedQuestion = serde_json::from_str(&response)?;
    Ok(decomposed.sub_questions)
}

pub async fn check_if_answer_is_complete(
    main_question: &str,
    global_summary: &str,
    model: &str,
    verbose: bool,
) -> Result<String, OllamaError> {
    let prompt = prompts::check_if_answer_is_complete_prompt(main_question, global_summary);

    #[derive(JsonSchema, Deserialize, serde::Serialize)]
    struct CompletenessDecision {
        decision: String,
    }

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<CompletenessDecision>()));
    let response = query_ollama(&prompt, model, Some(format), verbose).await?;

    let decision: CompletenessDecision = serde_json::from_str(&response)?;
    Ok(decision.decision.trim().to_lowercase())
}
