
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The main question/topic to research
    pub question: Option<String>,

    /// Maximum number of iterations
    #[arg(short = 'i', long, default_value_t = 3)]
    pub max_iterations: u8,
}

