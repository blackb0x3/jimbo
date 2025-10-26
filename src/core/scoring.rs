//! Scoring calculation engine
//!
//! This module handles the complex scoring logic for Balatro,
//! including base hand values, card bonuses, and joker effects.

use super::card::Card;
use super::hand::{Hand, HandType};
use super::joker::Joker;
use serde::{Deserialize, Serialize};

/// Result of a scoring calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoreResult {
    pub hand_type: HandType,
    pub chips: u32,
    pub mult: u32,
    pub score: u64,
    pub breakdown: ScoreBreakdown,
}

/// Detailed breakdown of how the score was calculated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub base_chips: u32,
    pub base_mult: u32,
    pub card_chips: u32,
    pub card_mult: u32,
    pub joker_chips: i32,
    pub joker_mult: i32,
    pub joker_mult_multiplier: f32,
}

/// The main scoring calculator
pub struct ScoreCalculator {
    jokers: Vec<Joker>,
}

impl ScoreCalculator {
    /// Creates a new score calculator with the given jokers
    pub fn new(jokers: Vec<Joker>) -> Self {
        Self { jokers }
    }

    /// Calculates the score for a given hand
    pub fn calculate(&self, hand: &Hand) -> ScoreResult {
        let hand_type = hand.evaluate();

        // Base values from hand type
        let base_chips = hand_type.base_chips();
        let base_mult = hand_type.base_mult();

        // Calculate card contributions
        let (card_chips, card_mult) = self.calculate_card_bonuses(&hand.cards);

        // Calculate joker contributions
        let (joker_chips, joker_mult, joker_mult_multiplier) =
            self.calculate_joker_bonuses(hand, hand_type);

        // Apply all modifiers
        let total_chips = (base_chips + card_chips).saturating_add_signed(joker_chips);
        let total_mult = (base_mult + card_mult).saturating_add_signed(joker_mult);

        // Apply multiplicative joker effects
        let final_mult = (total_mult as f32 * joker_mult_multiplier) as u32;

        // Final score: chips * mult
        let score = (total_chips as u64) * (final_mult as u64);

        ScoreResult {
            hand_type,
            chips: total_chips,
            mult: final_mult,
            score,
            breakdown: ScoreBreakdown {
                base_chips,
                base_mult,
                card_chips,
                card_mult,
                joker_chips,
                joker_mult,
                joker_mult_multiplier,
            },
        }
    }

    /// Calculates chip and mult bonuses from cards
    fn calculate_card_bonuses(&self, cards: &[Card]) -> (u32, u32) {
        let mut chips = 0u32;
        let mut mult = 0u32;

        for card in cards {
            // Base card value
            chips += card.base_chips();

            // Enhancement bonuses
            match card.enhancement {
                super::card::Enhancement::Bonus => chips += 30,
                super::card::Enhancement::Mult => mult += 4,
                super::card::Enhancement::Stone => chips += 50,
                _ => {} // Other enhancements handled elsewhere
            }

            // Edition bonuses
            match card.edition {
                super::card::Edition::Foil => chips += 50,
                super::card::Edition::Holographic => mult += 10,
                _ => {} // Polychrome is multiplicative, handled separately
            }
        }

        (chips, mult)
    }

    /// Calculates bonuses from jokers
    fn calculate_joker_bonuses(
        &self,
        _hand: &Hand,
        _hand_type: HandType,
    ) -> (i32, i32, f32) {
        let mut chips = 0i32;
        let mut mult = 0i32;
        let mut mult_multiplier = 1.0f32;

        for joker in &self.jokers {
            // Base joker effects
            chips += joker.kind.base_chips();
            mult += joker.kind.base_mult();

            // Joker edition effects
            match joker.edition {
                super::joker::JokerEdition::Foil => chips += 50,
                super::joker::JokerEdition::Holographic => mult += 10,
                super::joker::JokerEdition::Polychrome => mult_multiplier *= 1.5,
                _ => {}
            }

            // TODO: Implement conditional joker effects based on hand composition
            // This will be expanded as more jokers are implemented
        }

        (chips, mult, mult_multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::{Rank, Suit};
    use crate::core::joker::JokerKind;

    #[test]
    fn test_basic_scoring() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Spades),
        ];
        let hand = Hand::new(cards);
        let calculator = ScoreCalculator::new(vec![]);

        let result = calculator.calculate(&hand);

        assert_eq!(result.hand_type, HandType::Pair);
        assert_eq!(result.breakdown.base_chips, 10);
        assert_eq!(result.breakdown.base_mult, 2);
        assert_eq!(result.breakdown.card_chips, 22); // Two aces: 11 + 11
    }

    #[test]
    fn test_scoring_with_joker() {
        let cards = vec![
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
        ];
        let hand = Hand::new(cards);
        let jokers = vec![Joker::new(JokerKind::Joker)];
        let calculator = ScoreCalculator::new(jokers);

        let result = calculator.calculate(&hand);

        assert_eq!(result.breakdown.joker_mult, 4); // Basic Joker gives +4 mult
    }
}
