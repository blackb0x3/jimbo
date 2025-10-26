//! Game state configuration
//!
//! This module handles loading and saving complete game state configurations,
//! including decks, jokers, consumables, vouchers, and blind conditions.

use crate::core::joker::Joker;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Represents a complete game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Path to deck configuration file or inline deck definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deck_path: Option<String>,

    /// List of active jokers
    #[serde(default)]
    pub jokers: Vec<String>,

    /// List of available consumables (tarots, planets, spectrals)
    #[serde(default)]
    pub consumables: Vec<String>,

    /// List of purchased vouchers
    #[serde(default)]
    pub vouchers: Vec<String>,

    /// Current blind configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blind: Option<BlindConfig>,

    /// Optional seed for reproducibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    /// Current money
    #[serde(default)]
    pub money: u32,

    /// Current ante/round
    #[serde(default = "default_ante")]
    pub ante: u32,
}

fn default_ante() -> u32 {
    1
}

/// Configuration for a blind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlindConfig {
    /// Type of blind (small, big, boss)
    pub blind_type: BlindType,

    /// Name of the blind (for boss blinds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Required score to beat the blind
    pub score_required: u64,

    /// Special ability description (for boss blinds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability: Option<String>,
}

/// Type of blind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlindType {
    Small,
    Big,
    Boss,
}

impl GameState {
    /// Creates a new empty game state
    pub fn new() -> Self {
        Self {
            deck_path: None,
            jokers: Vec::new(),
            consumables: Vec::new(),
            vouchers: Vec::new(),
            blind: None,
            seed: None,
            money: 0,
            ante: 1,
        }
    }

    /// Loads a game state from a JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read game state from {:?}", path.as_ref()))?;

        let state: GameState = serde_json::from_str(&contents)
            .context("Failed to parse game state JSON")?;

        Ok(state)
    }

    /// Saves a game state to a JSON file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize game state")?;

        fs::write(&path, json)
            .with_context(|| format!("Failed to write game state to {:?}", path.as_ref()))?;

        Ok(())
    }

    /// Parses joker names into Joker objects
    pub fn parse_jokers(&self) -> Result<Vec<Joker>> {
        // TODO: Implement joker name parsing
        // For now, return empty vector
        Ok(Vec::new())
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl BlindConfig {
    /// Creates a new blind configuration
    pub fn new(blind_type: BlindType, score_required: u64) -> Self {
        Self {
            blind_type,
            name: None,
            score_required,
            ability: None,
        }
    }

    /// Creates a small blind with the given score requirement
    pub fn small(score_required: u64) -> Self {
        Self::new(BlindType::Small, score_required)
    }

    /// Creates a big blind with the given score requirement
    pub fn big(score_required: u64) -> Self {
        Self::new(BlindType::Big, score_required)
    }

    /// Creates a boss blind with the given name and score requirement
    pub fn boss(name: String, score_required: u64) -> Self {
        Self {
            blind_type: BlindType::Boss,
            name: Some(name),
            score_required,
            ability: None,
        }
    }

    /// Adds an ability description to this blind
    pub fn with_ability(mut self, ability: String) -> Self {
        self.ability = Some(ability);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let state = GameState::new();
        assert_eq!(state.ante, 1);
        assert_eq!(state.money, 0);
        assert!(state.jokers.is_empty());
    }

    #[test]
    fn test_blind_creation() {
        let blind = BlindConfig::small(300);
        assert_eq!(blind.blind_type, BlindType::Small);
        assert_eq!(blind.score_required, 300);

        let boss = BlindConfig::boss("The Hook".to_string(), 2000);
        assert_eq!(boss.blind_type, BlindType::Boss);
        assert_eq!(boss.name, Some("The Hook".to_string()));
    }

    #[test]
    fn test_serialization() {
        let state = GameState {
            deck_path: Some("standard.json".to_string()),
            jokers: vec!["Joker".to_string(), "Greedy_Joker".to_string()],
            blind: Some(BlindConfig::small(300)),
            seed: Some(12345),
            money: 100,
            ante: 2,
            ..Default::default()
        };

        let json = serde_json::to_string(&state).unwrap();
        let deserialized: GameState = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.jokers.len(), 2);
        assert_eq!(deserialized.seed, Some(12345));
    }
}
