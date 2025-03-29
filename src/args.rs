use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "gcomm",
    version,
    about = "Generate AI-powered Git commit messages from staged changes"
)]
pub struct Args {
    /// Model name to use with Ollama
    #[arg(short, long, default_value = "gemma3:latest")]
    pub model: String,

    /// Commit automatically without editing
    #[arg(long)]
    pub yolo: bool,

    // Open editor to edit commit message
    #[arg(long)]
    pub edit: bool,
}
