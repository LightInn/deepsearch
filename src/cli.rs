use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The main question/topic to research
    pub question: String,

    /// Maximum number of iterations
    #[arg(short = 'i', long, default_value_t = 3)]
    pub max_iterations: u8,

    /// Ollama model to use
    #[arg(short, long, default_value_t = String::from("gemma3:4b"))]
    pub model: String,
}
