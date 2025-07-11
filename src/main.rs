
use clap::Parser;
use colored::*;
use inquire::{Select, Text};
use std::io;

mod cli;
mod ollama;
mod orchestrator;
mod parser;
mod search;

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = cli::Cli::parse();

    // --- Interactive Model Selection ---
    let available_models = match ollama::list_ollama_models().await {
        Ok(models) => models,
        Err(e) => {
            eprintln!(
                "{}{}",
                "Error listing Ollama models: ".red().bold(),
                e.to_string().red()
            );
            eprintln!(
                "{}",
                "Please ensure Ollama is running and the model is available."
                    .red()
                    .bold()
            );
            return Ok(());
        }
    };

    if available_models.is_empty() {
        eprintln!("{}", "No Ollama models found.".red().bold());
        eprintln!(
            "{}",
            "Please pull a model with `ollama pull <model_name>`"
                .red()
                .bold()
        );
        return Ok(());
    }

    let model = Select::new("Select an Ollama model:", available_models)
        .prompt()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // --- Interactive Question ---
    let question = match cli.question {
        Some(q) => q,
        None => Text::new("What is your research question?")
            .prompt()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?,
    };

    println!(
        "{}{}{}",
        "🔍 Starting research for: ".bold().cyan(),
        question.bold().yellow(),
        "...".bold().cyan()
    );

    orchestrator::run(&question, cli.max_iterations, &model).await;

    Ok(())
}
