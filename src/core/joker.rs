//! Joker definitions and effects
//!
//! This module defines all joker types and their scoring modifications.
//! Each joker has unique effects that can modify chips, mult, or trigger
//! special behaviors during scoring.

use serde::{Deserialize, Serialize};

/// Represents a joker and its current state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Joker {
    pub kind: JokerKind,
    pub edition: JokerEdition,
    pub rarity: JokerRarity,
}

/// The type of joker and its effect
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JokerKind {
    // Basic jokers
    Joker,              // +4 mult
    GreedyJoker,        // Played cards with Diamond suit give +3 mult
    LustyJoker,         // Played cards with Heart suit give +3 mult
    WrathfulJoker,      // Played cards with Spade suit give +3 mult
    GluttonousJoker,    // Played cards with Club suit give +3 mult

    // Conditional jokers
    JollyJoker,         // +8 mult if played hand contains a Pair
    ZanyJoker,          // +12 mult if played hand contains a Three of a Kind
    MadJoker,           // +10 mult if played hand contains a Two Pair
    CrazyJoker,         // +12 mult if played hand contains a Straight
    DrollJoker,         // +10 mult if played hand contains a Flush

    // Multiplicative jokers
    Baron,              // x1.5 mult for each King in hand

    // TODO: Add more jokers as they are implemented
    // This is a placeholder structure to be expanded
}

/// Edition modifications for jokers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JokerEdition {
    None,
    Foil,        // +50 chips
    Holographic, // +10 mult
    Polychrome,  // x1.5 mult
    Negative,    // +1 joker slot (doesn't affect scoring directly)
}

/// Joker rarity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JokerRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl Joker {
    /// Creates a new joker with the given kind
    pub fn new(kind: JokerKind) -> Self {
        Self {
            kind,
            edition: JokerEdition::None,
            rarity: JokerRarity::Common,
        }
    }

    /// Creates a joker with a specific edition
    pub fn with_edition(mut self, edition: JokerEdition) -> Self {
        self.edition = edition;
        self
    }

    /// Creates a joker with a specific rarity
    pub fn with_rarity(mut self, rarity: JokerRarity) -> Self {
        self.rarity = rarity;
        self
    }
}

impl JokerKind {
    /// Returns the base chip bonus for this joker (if any)
    pub fn base_chips(&self) -> i32 {
        match self {
            _ => 0, // Most jokers don't add flat chips
        }
    }

    /// Returns the base mult bonus for this joker (if any)
    pub fn base_mult(&self) -> i32 {
        match self {
            JokerKind::Joker => 4,
            _ => 0, // Most jokers have conditional effects
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joker_creation() {
        let joker = Joker::new(JokerKind::Joker);
        assert_eq!(joker.kind, JokerKind::Joker);
        assert_eq!(joker.edition, JokerEdition::None);
    }

    #[test]
    fn test_base_joker_mult() {
        assert_eq!(JokerKind::Joker.base_mult(), 4);
    }
}
