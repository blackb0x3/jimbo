//! Hand evaluation and poker hand types
//!
//! This module handles evaluating collections of cards to determine
//! poker hand types and their base scoring values.

use super::card::{Card, Rank};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the type of poker hand
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,     // Balatro-specific (with wild cards)
    FlushHouse,      // Balatro-specific (flush + full house)
    FlushFive,       // Balatro-specific (five of a kind + flush)
}

impl HandType {
    /// Returns the base chips for this hand type
    pub fn base_chips(&self) -> u32 {
        match self {
            HandType::HighCard => 5,
            HandType::Pair => 10,
            HandType::TwoPair => 20,
            HandType::ThreeOfAKind => 30,
            HandType::Straight => 30,
            HandType::Flush => 35,
            HandType::FullHouse => 40,
            HandType::FourOfAKind => 60,
            HandType::StraightFlush => 100,
            HandType::FiveOfAKind => 120,
            HandType::FlushHouse => 140,
            HandType::FlushFive => 160,
        }
    }

    /// Returns the base multiplier for this hand type
    pub fn base_mult(&self) -> u32 {
        match self {
            HandType::HighCard => 1,
            HandType::Pair => 2,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::Straight => 4,
            HandType::Flush => 4,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 7,
            HandType::StraightFlush => 8,
            HandType::FiveOfAKind => 12,
            HandType::FlushHouse => 14,
            HandType::FlushFive => 16,
        }
    }
}

/// Represents a collection of cards that form a playable hand
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    /// Creates a new hand from a vector of cards
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    /// Evaluates the hand to determine its type
    pub fn evaluate(&self) -> HandType {
        if self.cards.is_empty() {
            return HandType::HighCard;
        }

        let is_flush = self.is_flush();
        let is_straight = self.is_straight();
        let rank_counts = self.rank_counts();

        // Check for special Balatro hands
        if let Some(hand_type) = self.check_special_hands(&rank_counts, is_flush, is_straight) {
            return hand_type;
        }

        // Check standard poker hands
        self.check_standard_hands(&rank_counts, is_flush, is_straight)
    }

    /// Checks for special Balatro-specific hand types
    fn check_special_hands(
        &self,
        rank_counts: &HashMap<Rank, usize>,
        is_flush: bool,
        _is_straight: bool,
    ) -> Option<HandType> {
        let max_count = rank_counts.values().max().copied().unwrap_or(0);

        // Flush Five: Five of a kind + flush
        if max_count >= 5 && is_flush {
            return Some(HandType::FlushFive);
        }

        // Flush House: Full house + flush
        if is_flush && self.is_full_house(rank_counts) {
            return Some(HandType::FlushHouse);
        }

        // Five of a Kind
        if max_count >= 5 {
            return Some(HandType::FiveOfAKind);
        }

        None
    }

    /// Checks for standard poker hand types
    fn check_standard_hands(
        &self,
        rank_counts: &HashMap<Rank, usize>,
        is_flush: bool,
        is_straight: bool,
    ) -> HandType {
        let max_count = rank_counts.values().max().copied().unwrap_or(0);
        let pair_count = rank_counts.values().filter(|&&count| count == 2).count();

        // Straight Flush
        if is_straight && is_flush {
            return HandType::StraightFlush;
        }

        // Four of a Kind
        if max_count == 4 {
            return HandType::FourOfAKind;
        }

        // Full House
        if self.is_full_house(rank_counts) {
            return HandType::FullHouse;
        }

        // Flush
        if is_flush {
            return HandType::Flush;
        }

        // Straight
        if is_straight {
            return HandType::Straight;
        }

        // Three of a Kind
        if max_count == 3 {
            return HandType::ThreeOfAKind;
        }

        // Two Pair
        if pair_count >= 2 {
            return HandType::TwoPair;
        }

        // Pair
        if max_count == 2 {
            return HandType::Pair;
        }

        // High Card
        HandType::HighCard
    }

    /// Checks if all cards are the same suit
    fn is_flush(&self) -> bool {
        if self.cards.len() < 5 {
            return false;
        }
        let first_suit = self.cards[0].suit;
        self.cards.iter().all(|card| card.suit == first_suit)
    }

    /// Checks if cards form a straight (consecutive ranks)
    fn is_straight(&self) -> bool {
        if self.cards.len() < 5 {
            return false;
        }

        let mut values: Vec<u8> = self.cards.iter().map(|card| card.rank.value()).collect();
        values.sort_unstable();
        values.dedup();

        if values.len() < 5 {
            return false;
        }

        // Check for consecutive values
        for window in values.windows(5) {
            if window[4] - window[0] == 4 {
                return true;
            }
        }

        // Check for Ace-low straight (A-2-3-4-5)
        if values.contains(&14) && values.contains(&2) && values.contains(&3)
            && values.contains(&4) && values.contains(&5) {
            return true;
        }

        false
    }

    /// Counts occurrences of each rank
    fn rank_counts(&self) -> HashMap<Rank, usize> {
        let mut counts = HashMap::new();
        for card in &self.cards {
            *counts.entry(card.rank).or_insert(0) += 1;
        }
        counts
    }

    /// Checks if hand is a full house (three of a kind + pair)
    fn is_full_house(&self, rank_counts: &HashMap<Rank, usize>) -> bool {
        let has_three = rank_counts.values().any(|&count| count == 3);
        let has_pair = rank_counts.values().any(|&count| count == 2);
        has_three && has_pair
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::Suit;

    #[test]
    fn test_hand_type_values() {
        assert_eq!(HandType::Pair.base_chips(), 10);
        assert_eq!(HandType::Pair.base_mult(), 2);
        assert_eq!(HandType::Flush.base_chips(), 35);
        assert_eq!(HandType::Flush.base_mult(), 4);
    }

    #[test]
    fn test_pair_evaluation() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
        ];
        let hand = Hand::new(cards);
        assert_eq!(hand.evaluate(), HandType::Pair);
    }

    #[test]
    fn test_flush_evaluation() {
        let cards = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
        ];
        let hand = Hand::new(cards);
        assert_eq!(hand.evaluate(), HandType::Flush);
    }
}
