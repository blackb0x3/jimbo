//! Optimal play solver
//!
//! This module contains the algorithm for finding the highest-scoring
//! combination of cards from a given hand.

use super::card::Card;
use super::hand::Hand;
use super::scoring::{ScoreCalculator, ScoreResult};

/// The solver finds optimal plays from a given hand
pub struct Solver {
    calculator: ScoreCalculator,
}

impl Solver {
    /// Creates a new solver with the given score calculator
    pub fn new(calculator: ScoreCalculator) -> Self {
        Self { calculator }
    }

    /// Finds the best play from the given cards
    pub fn solve(&self, cards: &[Card]) -> SolverResult {
        if cards.is_empty() {
            return SolverResult {
                best_hand: Hand::new(vec![]),
                best_score: None,
                alternatives: vec![],
            };
        }

        let mut results: Vec<(Hand, ScoreResult)> = Vec::new();

        // Generate all possible hand combinations (1 to 5 cards)
        for hand_size in 1..=5.min(cards.len()) {
            let combinations = Self::generate_combinations(cards, hand_size);

            for combo in combinations {
                let hand = Hand::new(combo);
                let score = self.calculator.calculate(&hand);
                results.push((hand, score));
            }
        }

        // Sort by score (descending)
        results.sort_by(|a, b| b.1.score.cmp(&a.1.score));

        // Extract best and alternatives
        let best = results.first().cloned();
        let alternatives: Vec<_> = results.into_iter().skip(1).take(3).collect();

        SolverResult {
            best_hand: best.as_ref().map(|(h, _)| h.clone()).unwrap_or_else(|| Hand::new(vec![])),
            best_score: best.map(|(_, s)| s),
            alternatives,
        }
    }

    /// Generates all combinations of cards of a given size
    fn generate_combinations(cards: &[Card], size: usize) -> Vec<Vec<Card>> {
        let mut results = Vec::new();
        let mut current = Vec::new();
        Self::generate_combinations_recursive(cards, size, 0, &mut current, &mut results);
        results
    }

    /// Recursive helper for generating combinations
    fn generate_combinations_recursive(
        cards: &[Card],
        size: usize,
        start: usize,
        current: &mut Vec<Card>,
        results: &mut Vec<Vec<Card>>,
    ) {
        if current.len() == size {
            results.push(current.clone());
            return;
        }

        for i in start..cards.len() {
            current.push(cards[i].clone());
            Self::generate_combinations_recursive(cards, size, i + 1, current, results);
            current.pop();
        }
    }
}

/// Result from the solver containing the best play and alternatives
#[derive(Debug, Clone)]
pub struct SolverResult {
    pub best_hand: Hand,
    pub best_score: Option<ScoreResult>,
    pub alternatives: Vec<(Hand, ScoreResult)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::{Rank, Suit};

    #[test]
    fn test_combination_generation() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
        ];

        let combos = Solver::generate_combinations(&cards, 2);
        assert_eq!(combos.len(), 3); // C(3,2) = 3
    }

    #[test]
    fn test_solver_basic() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
        ];

        let calculator = ScoreCalculator::new(vec![]);
        let solver = Solver::new(calculator);
        let result = solver.solve(&cards);

        assert!(result.best_score.is_some());
        // Should find a valid hand (pair would be 2 cards, but solver might find a better combination)
        assert!(!result.best_hand.cards.is_empty());
    }
}
