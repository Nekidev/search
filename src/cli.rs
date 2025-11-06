use clap::Parser;

#[derive(Parser, Clone)]
#[command(name = "search")]
#[command(about = "A simple TUI search application", long_about = None)]
pub struct Query {
    pub query: String,

    #[arg(
        short,
        long,
        help = "Your Google Custom Search JSON API key",
        env = "GOOGLE_API_KEY"
    )]
    pub api_key: String,

    #[arg(
        short,
        long,
        help = "Your Google Custom Search Engine ID",
        env = "GOOGLE_CX"
    )]
    pub cx: String,

    #[arg(short, long, help = "Enable safe search")]
    pub safe: bool,
}
