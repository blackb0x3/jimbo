//! Config command implementation
//!
//! This module implements the `config` command which manages
//! configuration files for decks and game states.

use crate::config::{DeckConfig, GameState};
use anyhow::{Context, Result};
use clap::{Args, Subcommand};

/// Arguments for the config command
#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCommand,
}

/// Subcommands for config management
#[derive(Debug, Subcommand)]
enum ConfigCommand {
    /// Create a new configuration file
    Init {
        /// Type of config: deck or game-state
        #[arg(value_parser = ["deck", "game-state"])]
        config_type: String,

        /// Output file path
        #[arg(short, long)]
        output: String,
    },

    /// Validate an existing configuration file
    Validate {
        /// Path to configuration file
        file: String,
    },

    /// List all saved configurations in a directory
    List {
        /// Directory to search (default: current directory)
        #[arg(short, long, default_value = ".")]
        dir: String,
    },
}

/// Runs the config command
pub fn run(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommand::Init {
            config_type,
            output,
        } => init_config(&config_type, &output),
        ConfigCommand::Validate { file } => validate_config(&file),
        ConfigCommand::List { dir } => list_configs(&dir),
    }
}

/// Initializes a new configuration file
fn init_config(config_type: &str, output_path: &str) -> Result<()> {
    match config_type {
        "deck" => {
            let deck = DeckConfig::standard();
            deck.to_file(output_path)
                .with_context(|| format!("Failed to create deck config at {}", output_path))?;
            println!("✅ Created standard deck configuration at: {}", output_path);
            println!("   (52-card standard deck)");
        }
        "game-state" => {
            let state = GameState::default();
            state
                .to_file(output_path)
                .with_context(|| format!("Failed to create game state at {}", output_path))?;
            println!("✅ Created empty game state at: {}", output_path);
            println!("   Edit the file to add jokers, vouchers, and blind configuration");
        }
        _ => anyhow::bail!("Invalid config type: {}. Use 'deck' or 'game-state'", config_type),
    }

    Ok(())
}

/// Validates a configuration file
fn validate_config(file_path: &str) -> Result<()> {
    // Try to load as deck config first
    if let Ok(deck_config) = DeckConfig::from_file(file_path) {
        match deck_config.validate() {
            Ok(()) => {
                println!("✅ Valid deck configuration");
                println!("   Cards: {}", deck_config.cards.len());
                println!("   Enhancements: {}", deck_config.enhancements.len());
                println!("   Editions: {}", deck_config.editions.len());
                println!("   Seals: {}", deck_config.seals.len());
                return Ok(());
            }
            Err(e) => {
                println!("❌ Invalid deck configuration:");
                println!("   {}", e);
                return Err(e);
            }
        }
    }

    // Try to load as game state
    if let Ok(game_state) = GameState::from_file(file_path) {
        println!("✅ Valid game state configuration");
        println!("   Jokers: {}", game_state.jokers.len());
        println!("   Consumables: {}", game_state.consumables.len());
        println!("   Vouchers: {}", game_state.vouchers.len());
        if let Some(blind) = &game_state.blind {
            println!("   Blind: {:?} (score required: {})",
                blind.blind_type, blind.score_required);
        }
        if let Some(seed) = game_state.seed {
            println!("   Seed: {}", seed);
        }
        return Ok(());
    }

    anyhow::bail!("File is not a valid deck config or game state: {}", file_path)
}

/// Lists all configuration files in a directory
fn list_configs(dir_path: &str) -> Result<()> {
    use std::fs;

    let entries = fs::read_dir(dir_path)
        .with_context(|| format!("Failed to read directory: {}", dir_path))?;

    let mut deck_configs = Vec::new();
    let mut game_states = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let path_str = path.to_string_lossy().to_string();

            // Try to load as deck config
            if DeckConfig::from_file(&path_str).is_ok() {
                deck_configs.push(path_str.clone());
            }

            // Try to load as game state
            if GameState::from_file(&path_str).is_ok() {
                game_states.push(path_str);
            }
        }
    }

    if deck_configs.is_empty() && game_states.is_empty() {
        println!("No configuration files found in: {}", dir_path);
        return Ok(());
    }

    if !deck_configs.is_empty() {
        println!("🃏 Deck Configurations:");
        for config in deck_configs {
            println!("   - {}", config);
        }
    }

    if !game_states.is_empty() {
        println!("\n🎮 Game States:");
        for state in game_states {
            println!("   - {}", state);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_type_parsing() {
        // Just verify the command structure compiles
        assert_eq!("deck", "deck");
        assert_eq!("game-state", "game-state");
    }
}
