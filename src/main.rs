use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jimbo")]
#[command(version)]
#[command(about = "Your personal Balatro strategist", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyzes your hand and finds the optimal play
    Solve(jimbo::cli::solve::SolveArgs),

    /// Runs multiple simulations to find average/best-case scores
    Simulate(jimbo::cli::simulate::SimulateArgs),

    /// Launches the interactive terminal user interface
    Tui,

    /// Manage configuration files for decks and presets
    Config(jimbo::cli::config::ConfigArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Solve(args) => jimbo::cli::solve::run(args),
        Commands::Simulate(args) => jimbo::cli::simulate::run(args),
        Commands::Tui => jimbo::tui::run(),
        Commands::Config(args) => jimbo::cli::config::run(args),
    }
}
