mod commands;

use clap::{Parser, Subcommand};

/// mc — a multi-command toolkit
#[derive(Parser)]
#[command(name = "mc", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the current unix timestamp
    Ts(commands::ts::TsArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ts(args) => commands::ts::run(args),
    }
}
