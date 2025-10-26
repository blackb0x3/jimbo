//! Deck configuration and serialization
//!
//! This module handles loading and saving deck configurations from JSON files,
//! including card enhancements, editions, and seals.

use crate::core::card::{Card, Edition, Enhancement, Rank, Seal, Suit};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a complete deck configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckConfig {
    /// The cards in the deck
    pub cards: Vec<CardDefinition>,

    /// Card enhancements mapped by card ID (e.g., "AH" for Ace of Hearts)
    #[serde(default)]
    pub enhancements: HashMap<String, Enhancement>,

    /// Card editions mapped by card ID
    #[serde(default)]
    pub editions: HashMap<String, Edition>,

    /// Card seals mapped by card ID
    #[serde(default)]
    pub seals: HashMap<String, Seal>,
}

/// A card definition in the configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDefinition {
    pub rank: String,
    pub suit: String,
}

impl DeckConfig {
    /// Creates a new empty deck configuration
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            enhancements: HashMap::new(),
            editions: HashMap::new(),
            seals: HashMap::new(),
        }
    }

    /// Creates a standard 52-card deck
    pub fn standard() -> Self {
        let ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];

        let mut cards = Vec::new();
        for suit in suits {
            for rank in ranks {
                cards.push(CardDefinition {
                    rank: rank.to_string(),
                    suit: suit.to_string(),
                });
            }
        }

        Self {
            cards,
            enhancements: HashMap::new(),
            editions: HashMap::new(),
            seals: HashMap::new(),
        }
    }

    /// Loads a deck configuration from a JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read deck config from {:?}", path.as_ref()))?;

        let config: DeckConfig = serde_json::from_str(&contents)
            .context("Failed to parse deck config JSON")?;

        config.validate()?;
        Ok(config)
    }

    /// Saves a deck configuration to a JSON file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.validate()?;

        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize deck config")?;

        fs::write(&path, json)
            .with_context(|| format!("Failed to write deck config to {:?}", path.as_ref()))?;

        Ok(())
    }

    /// Validates the deck configuration
    pub fn validate(&self) -> Result<()> {
        // Check that we have at least one card
        anyhow::ensure!(!self.cards.is_empty(), "Deck must contain at least one card");

        // Validate each card definition
        for card_def in &self.cards {
            Self::parse_rank(&card_def.rank)
                .with_context(|| format!("Invalid rank: {}", card_def.rank))?;
            Self::parse_suit(&card_def.suit)
                .with_context(|| format!("Invalid suit: {}", card_def.suit))?;
        }

        Ok(())
    }

    /// Converts this configuration into actual Card objects
    pub fn to_cards(&self) -> Result<Vec<Card>> {
        let mut cards = Vec::new();

        for card_def in &self.cards {
            let rank = Self::parse_rank(&card_def.rank)?;
            let suit = Self::parse_suit(&card_def.suit)?;
            let card_id = Self::make_card_id(rank, suit);

            let mut card = Card::new(rank, suit);

            // Apply enhancements
            if let Some(enhancement) = self.enhancements.get(&card_id) {
                card = card.with_enhancement(*enhancement);
            }

            // Apply editions
            if let Some(edition) = self.editions.get(&card_id) {
                card = card.with_edition(*edition);
            }

            // Apply seals
            if let Some(seal) = self.seals.get(&card_id) {
                card = card.with_seal(*seal);
            }

            cards.push(card);
        }

        Ok(cards)
    }

    /// Parses a rank string into a Rank enum
    fn parse_rank(s: &str) -> Result<Rank> {
        match s {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "10" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => anyhow::bail!("Unknown rank: {}", s),
        }
    }

    /// Parses a suit string into a Suit enum
    fn parse_suit(s: &str) -> Result<Suit> {
        match s {
            "Hearts" | "H" => Ok(Suit::Hearts),
            "Diamonds" | "D" => Ok(Suit::Diamonds),
            "Clubs" | "C" => Ok(Suit::Clubs),
            "Spades" | "S" => Ok(Suit::Spades),
            _ => anyhow::bail!("Unknown suit: {}", s),
        }
    }

    /// Creates a card ID string (e.g., "AH" for Ace of Hearts)
    fn make_card_id(rank: Rank, suit: Suit) -> String {
        let rank_str = match rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };

        let suit_str = match suit {
            Suit::Hearts => "H",
            Suit::Diamonds => "D",
            Suit::Clubs => "C",
            Suit::Spades => "S",
        };

        format!("{}{}", rank_str, suit_str)
    }
}

impl Default for DeckConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_deck() {
        let deck = DeckConfig::standard();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_deck_validation() {
        let deck = DeckConfig::standard();
        assert!(deck.validate().is_ok());
    }

    #[test]
    fn test_to_cards() {
        let deck = DeckConfig::standard();
        let cards = deck.to_cards().unwrap();
        assert_eq!(cards.len(), 52);
    }
}
