use clap::{Parser, Subcommand};

mod downloader;
mod runner;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(long_about = "Downloads input files")]
    DlInput {
        #[arg(long_help = "Your session cookie")]
        session_cookie: String,

        #[arg(short, long, long_help = "Until which day to download inputs")]
        until_day: Option<i32>,
    },

    #[command(long_about = "Runs the solutions")]
    Run,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::DlInput {
            session_cookie,
            until_day,
        } => downloader::download_input(session_cookie, until_day),
        Commands::Run => runner::run(),
    }
}
