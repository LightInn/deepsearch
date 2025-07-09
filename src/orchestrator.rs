use crate::{search, ollama, parser};
use colored::*;

pub async fn run(question: &str, max_iterations: u8, model: &str) {
    let mut global_summary = String::new();
    let mut iteration = 0;

    while iteration < max_iterations {
        iteration += 1;
        println!("{}{}{}", "

--- Iteration ".bold().blue(), iteration.to_string().bold().blue(), " ---".bold().blue());

        println!("{}", "Generating sub-question...".yellow());
        let sub_question = match generate_next_sub_question(question, &global_summary, model).await {
            Ok(sq) => sq,
            Err(e) => {
                eprintln!("Error generating sub-question: {}", e);
                break;
            }
        };

        println!("{}{}", "Sub-question: ".green(), sub_question.green());
        println!("{}", "Performing search...".yellow());
        let search_results = match perform_search(&sub_question, model).await {
            Ok(results) => results,
            Err(e) => {
                eprintln!("Error performing search: {}", e);
                continue;
            }
        };

        println!("{}", "Filtering search results...".yellow());
        let filtered_results = match ollama::filter_search_results(&sub_question, &search_results, model).await {
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
                    match ollama::summarize_text(&page_text, &sub_question, model).await {
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

        if should_stop(&global_summary, question, model).await {
            println!("{}", "Stopping condition met.".blue());
            break;
        }
    }

    println!("{}", "Evaluating completeness and generating final answer...".yellow());
    match ollama::evaluate_completeness_and_answer(question, &global_summary, model).await {
        Ok(final_answer) => {
            println!("{}{}{}", "\nFinal Answer:".bold().green(), "\n".bold().green(), final_answer.bold().green());
        }
        Err(e) => {
            eprintln!("Error generating final answer: {}", e);
        }
    }
}

async fn generate_next_sub_question(main_question: &str, context: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = if context.is_empty() {
        format!(r#"
You are a meticulous research assistant. Your task is to generate a specific and relevant sub-question to help answer the main question.
First, identify the core event or entity in the main question. Then, formulate a sub-question to gather initial information about that event or entity.
Pay strict attention to the details provided in the main question, such as dates, names, and locations. Do not invent or change these details.
The sub-question should be a single, focused query that can be effectively used for a web search.
Respond with a JSON object containing a single key "question". Do NOT wrap the JSON in a Markdown code block.

Example:
{{
  "question": "What were the key findings of the 2023 IPCC report on climate change?"
}}

Main Question: '{}'
"#, main_question)
    } else {
        format!(r#"
You are a meticulous research assistant. Based on the main question and the research context provided below, generate the next single, specific sub-question to continue the research.
Pay strict attention to the details provided in the main question and context, such as dates, names, and locations. Do not invent or change these details.
The sub-question should be a focused query suitable for a web search.
Respond with a JSON object containing a single key "question". Do NOT wrap the JSON in a Markdown code block.

Main Question: '{}'

Research Context:
{}
"#, main_question, context)
    };

    let response = ollama::query_ollama(&prompt, model).await?;
    let json_response: serde_json::Value = serde_json::from_str(&response)?;
    let sub_question = json_response["question"].as_str().ok_or("Failed to extract question from JSON")?.to_string();
    Ok(sub_question)
}

async fn should_stop(global_summary: &str, main_question: &str, model: &str) -> bool {
    let prompt = format!("Based on the research summary so far:
{}

Can you now provide a comprehensive answer to the main question: '{}'? Respond with 'Yes' or 'No'.", global_summary, main_question);
    
    match ollama::query_ollama(&prompt, model).await {
        Ok(response) => response.trim().to_lowercase().starts_with("yes"),
        Err(_) => false, // If Ollama fails, assume we can't answer yet
    }
}

pub async fn perform_search(query: &str, model: &str) -> Result<Vec<search::SearchResult>, Box<dyn std::error::Error>> {
    let tool = ollama::decide_search_tool(query, model).await?;
    let results = if tool.contains("wikipedia") {
        search::search_wikipedia(query).await?
    } else {
        search::search_duckduckgo(query).await?
    };

    let filtered_results = results.into_iter().filter(|r| {
        !r.url.contains("youtube.com") && !r.url.contains("reddit.com")
    }).collect();

    Ok(filtered_results)
}