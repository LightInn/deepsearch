use crate::{search, ollama, parser, prompts};
use colored::*;
use serde::Deserialize;
use ollama_rs::generation::parameters::{FormatType, JsonSchema, JsonStructure};

pub async fn run(question: &str, max_iterations: u8, model: &str, verbose: bool) {
    let mut global_summary = String::new();
    let mut iteration = 0;

    while iteration < max_iterations {
        iteration += 1;
        println!("{}{}{}", "

--- Iteration ".bold().blue(), iteration.to_string().bold().blue(), " ---".bold().blue());

        println!("{}", "Generating sub-question...".yellow());
        let sub_question = match generate_next_sub_question(question, &global_summary, model, verbose).await {
            Ok(sq) => sq,
            Err(e) => {
                eprintln!("Error generating sub-question: {}", e);
                break;
            }
        };

        println!("{}{}", "Sub-question: ".green(), sub_question.green());
        let (search_results, tool_name) = match perform_search(&sub_question, model, verbose).await {
            Ok((results, tool_name)) => (results, tool_name),
            Err(e) => {
                eprintln!("Error performing search: {}", e);
                continue;
            }
        };
        println!("{}{}{}", "Performing search...".yellow(), " ", tool_name.yellow());

        println!("{}{}{}", "Filtering ".yellow(), search_results.len().to_string().yellow(), " search results...".yellow());
        let filtered_results = match ollama::filter_search_results(&sub_question, &search_results, model, verbose).await {
            Ok(results) => results,
            Err(e) => {
                eprintln!("Error filtering search results: {}", e);
                continue;
            }
        };

        for result in filtered_results {
            println!("{}{}", "Fetching and summarizing: ".yellow(), result.url.yellow());
            match parser::fetch_and_parse_html(&result.url).await {
                Ok(page_text) => {
                    match ollama::summarize_text(&page_text, &sub_question, model, verbose).await {
                        Ok(summary) => {
                            global_summary.push_str(&summary);
                            global_summary.push_str("\n\n");
                        }
                        Err(e) => {
                            eprintln!("Error summarizing text: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching and parsing HTML: {}", e);
                }
            }
        }

        if should_stop(&global_summary, question, model, verbose).await {
            println!("{}", "Stopping condition met.".blue());
            break;
        }
    }

    println!("{}", "Evaluating completeness and generating final answer...".yellow());
    match ollama::evaluate_completeness_and_answer(question, &global_summary, model, verbose).await {
        Ok(final_answer) => {
            println!("{}{}{}", "\nFinal Answer:".bold().green(), "\n".bold().green(), final_answer.bold().green());
        }
        Err(e) => {
            eprintln!("Error generating final answer: {}", e);
        }
    }
}

async fn generate_next_sub_question(
    main_question: &str,
    context: &str,
    model: &str,
    verbose: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    #[derive(JsonSchema, Deserialize)]
    struct SubQuestionOutput {
        question: String,
    }

    let prompt = if context.is_empty() {
        prompts::decompose_question_prompt_initial(main_question)
    } else {
        prompts::decompose_question_prompt_iterative(main_question, context)
    };

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<SubQuestionOutput>()));
    let response = ollama::query_ollama(&prompt, model, Some(format), verbose).await?;
    let sub_question_output: SubQuestionOutput = serde_json::from_str(&response)?;
    Ok(sub_question_output.question)
}

async fn should_stop(global_summary: &str, main_question: &str, model: &str, verbose: bool) -> bool {
    #[derive(JsonSchema, Deserialize)]
    struct StopDecision {
        decision: String,
    }

    let prompt = prompts::check_if_answer_is_complete_prompt(global_summary, main_question);

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<StopDecision>()));
    match ollama::query_ollama(&prompt, model, Some(format), verbose).await {
        Ok(response) => {
            let decision: StopDecision = serde_json::from_str(&response).unwrap_or_else(|_| StopDecision { decision: "no".to_string() });
            decision.decision.trim().to_lowercase().starts_with("yes")
        },
        Err(_) => false, // If Ollama fails, assume we can't answer yet
    }
}

pub async fn perform_search(query: &str, model: &str, verbose: bool) -> Result<(Vec<search::SearchResult>, String), Box<dyn std::error::Error>> {
    let tool = ollama::decide_search_tool(query, model, verbose).await?;
    let results = if tool.contains("wikipedia") {
        search::search_wikipedia(query).await?
    } else {
        search::search_duckduckgo(query).await?
    };

    let filtered_results = results.into_iter().filter(|r| {
        !r.url.contains("youtube.com") && !r.url.contains("reddit.com")
    }).collect();

    Ok((filtered_results, tool))
}