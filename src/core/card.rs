//! Card representations and properties
//!
//! This module defines the core Card type along with its enhancements,
//! editions, ranks, and suits as they appear in Balatro.

use serde::{Deserialize, Serialize};

/// Represents a playing card rank
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

/// Represents a playing card suit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

/// Card enhancements that modify scoring
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Enhancement {
    None,
    Bonus,      // +30 chips
    Mult,       // +4 mult
    Wild,       // Can be any suit
    Glass,      // x2 mult, 1/4 chance to destroy
    Steel,      // x1.5 mult while in hand
    Stone,      // +50 chips, no rank
    Gold,       // +$3 at end of round
    Lucky,      // 1/5 chance for +20 mult or $20
}

/// Card editions that provide special effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Edition {
    None,
    Foil,        // +50 chips
    Holographic, // +10 mult
    Polychrome,  // x1.5 mult
    Negative,    // +1 joker slot
}

/// Represents a single playing card with optional modifications
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub enhancement: Enhancement,
    pub edition: Edition,
    pub seal: Option<Seal>,
}

/// Card seals that trigger special effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Seal {
    Gold,   // +$3 when played
    Red,    // Retrigger card
    Blue,   // Creates Planet card if held in hand at end of round
    Purple, // Creates Tarot card when discarded
}

impl Card {
    /// Creates a new basic card without enhancements or editions
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self {
            rank,
            suit,
            enhancement: Enhancement::None,
            edition: Edition::None,
            seal: None,
        }
    }

    /// Creates a card with an enhancement
    pub fn with_enhancement(mut self, enhancement: Enhancement) -> Self {
        self.enhancement = enhancement;
        self
    }

    /// Creates a card with an edition
    pub fn with_edition(mut self, edition: Edition) -> Self {
        self.edition = edition;
        self
    }

    /// Creates a card with a seal
    pub fn with_seal(mut self, seal: Seal) -> Self {
        self.seal = Some(seal);
        self
    }

    /// Returns the base chip value of the card
    pub fn base_chips(&self) -> u32 {
        match self.rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        }
    }
}

impl Rank {
    /// Returns numeric value for rank comparison (for straights)
    pub fn value(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(Rank::Ace, Suit::Hearts);
        assert_eq!(card.rank, Rank::Ace);
        assert_eq!(card.suit, Suit::Hearts);
        assert_eq!(card.enhancement, Enhancement::None);
    }

    #[test]
    fn test_base_chips() {
        assert_eq!(Card::new(Rank::Ace, Suit::Hearts).base_chips(), 11);
        assert_eq!(Card::new(Rank::King, Suit::Spades).base_chips(), 10);
        assert_eq!(Card::new(Rank::Five, Suit::Diamonds).base_chips(), 5);
    }
}
