use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    DlInput {
        #[arg(long_help = "Your session cookie")]
        session_cookie: String,

        #[arg(short, long, long_help = "Until which day to download inputs")]
        until_day: Option<i32>,
    },
    Run,
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli)
}
