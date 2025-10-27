//! Simulate command implementation
//!
//! This module implements the `simulate` command which runs multiple
//! simulations to evaluate build performance.

use crate::config::DeckConfig;
use crate::core::{
    create_standard_deck, ScoreCalculator, SimulationConfig, Simulator, Solver,
};
use anyhow::{Context, Result};
use clap::Args;

/// Arguments for the simulate command
#[derive(Debug, Args)]
pub struct SimulateArgs {
    /// Number of simulation runs (default: 1000)
    #[arg(long, default_value = "1000")]
    runs: usize,

    /// Path to deck configuration file (default: standard 52-card deck)
    #[arg(long)]
    deck: Option<String>,

    /// Comma-separated list of jokers (e.g., "Joker,GreedyJoker")
    #[arg(long, value_delimiter = ',')]
    jokers: Vec<String>,

    /// Hand size to draw (default: 8)
    #[arg(long, default_value = "8")]
    hand_size: usize,

    /// Optional seed for reproducible simulations
    #[arg(long)]
    seed: Option<u64>,

    /// Output format: summary (default), detailed, csv
    #[arg(long, default_value = "summary")]
    output: OutputFormat,
}

/// Output format for the simulate command
#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Summary,
    Detailed,
    Csv,
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "summary" => Ok(OutputFormat::Summary),
            "detailed" => Ok(OutputFormat::Detailed),
            "csv" => Ok(OutputFormat::Csv),
            _ => anyhow::bail!(
                "Invalid output format: {}. Use 'summary', 'detailed', or 'csv'",
                s
            ),
        }
    }
}

/// Runs the simulate command
pub fn run(args: SimulateArgs) -> Result<()> {
    // Load or create deck
    let deck = if let Some(deck_path) = &args.deck {
        let deck_config = DeckConfig::from_file(deck_path)
            .with_context(|| format!("Failed to load deck config from {}", deck_path))?;
        deck_config.to_cards()?
    } else {
        create_standard_deck()
    };

    // Parse jokers (for now, empty)
    let jokers = Vec::new(); // TODO: Parse joker names

    // Create score calculator, solver, and simulator
    let calculator = ScoreCalculator::new(jokers);
    let solver = Solver::new(calculator);
    let simulator = Simulator::new(solver);

    // Configure simulation
    let config = SimulationConfig {
        deck,
        hand_size: args.hand_size,
        num_runs: args.runs,
        seed: args.seed,
    };

    // Run simulation
    println!("Running {} simulations...", args.runs);
    let result = simulator.simulate(config);

    // Display results based on output format
    match args.output {
        OutputFormat::Summary => display_summary(&result, &args),
        OutputFormat::Detailed => display_detailed(&result, &args),
        OutputFormat::Csv => display_csv(&result),
    }

    Ok(())
}

/// Displays results in summary format
fn display_summary(result: &crate::core::simulator::SimulationResult, args: &SimulateArgs) {
    println!("\n📊 Simulation Results ({} runs):", result.num_runs);
    println!("  Mean Score:   {:.2}", result.mean_score);
    println!("  Median Score: {}", result.median_score);
    println!("  Min Score:    {}", result.min_score);
    println!("  Max Score:    {}", result.max_score);
    println!("\n  Percentiles:");
    println!("    25th: {}", result.percentile_25);
    println!("    75th: {}", result.percentile_75);
    println!("    95th: {}", result.percentile_95);

    if let Some(seed) = args.seed {
        println!("\n  🎲 Seed: {} (reproducible)", seed);
    }
}

/// Displays results in detailed format
fn display_detailed(result: &crate::core::simulator::SimulationResult, args: &SimulateArgs) {
    println!("\n📊 Detailed Simulation Results");
    println!("═══════════════════════════════");
    println!("Configuration:");
    println!("  Runs:       {}", result.num_runs);
    println!("  Hand Size:  {}", args.hand_size);
    if let Some(seed) = args.seed {
        println!("  Seed:       {}", seed);
    }
    println!("\nScore Statistics:");
    println!("  Mean:       {:.2}", result.mean_score);
    println!("  Median:     {}", result.median_score);
    println!("  Min:        {}", result.min_score);
    println!("  Max:        {}", result.max_score);
    println!("  Range:      {}", result.max_score - result.min_score);
    println!("\nPercentile Distribution:");
    println!("  25th:       {}", result.percentile_25);
    println!("  50th:       {} (median)", result.median_score);
    println!("  75th:       {}", result.percentile_75);
    println!("  95th:       {}", result.percentile_95);
    println!("\nInterquartile Range (IQR):");
    println!("  IQR:        {}", result.percentile_75 - result.percentile_25);
}

/// Displays results in CSV format
fn display_csv(result: &crate::core::simulator::SimulationResult) {
    println!("num_runs,mean_score,median_score,min_score,max_score,p25,p75,p95");
    println!(
        "{},{:.2},{},{},{},{},{},{}",
        result.num_runs,
        result.mean_score,
        result.median_score,
        result.min_score,
        result.max_score,
        result.percentile_25,
        result.percentile_75,
        result.percentile_95
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_parsing() {
        assert!(matches!(
            "summary".parse::<OutputFormat>().unwrap(),
            OutputFormat::Summary
        ));
        assert!(matches!(
            "detailed".parse::<OutputFormat>().unwrap(),
            OutputFormat::Detailed
        ));
        assert!(matches!(
            "csv".parse::<OutputFormat>().unwrap(),
            OutputFormat::Csv
        ));
        assert!("invalid".parse::<OutputFormat>().is_err());
    }
}
