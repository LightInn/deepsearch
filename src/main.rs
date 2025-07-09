use clap::Parser;
use std::io;
use colored::*;

mod cli;
mod orchestrator;
mod ollama;
mod search;
mod parser;

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = cli::Cli::parse();

    println!("{}{}{}", "🔍 Starting research for: ".bold().cyan(), cli.question.bold().yellow(), "...".bold().cyan());

    // Validate Ollama model
    let available_models = match ollama::list_ollama_models().await {
        Ok(models) => models,
        Err(e) => {
            eprintln!("{}{}", "Error listing Ollama models: ".red().bold(), e.to_string().red());
            eprintln!("{}", "Please ensure Ollama is running and the model is available.".red().bold());
            return Ok(());
        }
    };

    if !available_models.contains(&cli.model) {
        eprintln!("{}{}{}", "Error: Model ".red().bold(), cli.model.red().bold(), " not found.".red().bold());
        eprintln!("{}", "Available models:".yellow().bold());
        for model in available_models {
            eprintln!("  - {}", model.yellow());
        }
        return Ok(());
    }

    orchestrator::run(&cli.question, cli.max_iterations, &cli.model).await;

    Ok(())
}