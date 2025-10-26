//! Monte Carlo simulator for testing builds
//!
//! This module runs multiple simulations with random hands to evaluate
//! the performance of different joker builds and deck configurations.

use super::card::{Card, Rank, Suit};
use super::solver::Solver;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

/// Configuration for a simulation run
pub struct SimulationConfig {
    pub deck: Vec<Card>,
    pub hand_size: usize,
    pub num_runs: usize,
    pub seed: Option<u64>,
}

/// Statistics from a simulation run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub num_runs: usize,
    pub mean_score: f64,
    pub median_score: u64,
    pub min_score: u64,
    pub max_score: u64,
    pub percentile_25: u64,
    pub percentile_75: u64,
    pub percentile_95: u64,
}

/// The simulator runs multiple hands and collects statistics
pub struct Simulator {
    solver: Solver,
}

impl Simulator {
    /// Creates a new simulator with the given solver
    pub fn new(solver: Solver) -> Self {
        Self { solver }
    }

    /// Runs a simulation with the given configuration
    pub fn simulate(&self, config: SimulationConfig) -> SimulationResult {
        let mut rng = self.create_rng(config.seed);
        let mut scores: Vec<u64> = Vec::with_capacity(config.num_runs);

        for _ in 0..config.num_runs {
            let hand = self.draw_random_hand(&config.deck, config.hand_size, &mut rng);
            let result = self.solver.solve(&hand);

            if let Some(score_result) = result.best_score {
                scores.push(score_result.score);
            } else {
                scores.push(0);
            }
        }

        self.calculate_statistics(scores, config.num_runs)
    }

    /// Creates a deterministic or random RNG based on seed
    fn create_rng(&self, seed: Option<u64>) -> ChaCha8Rng {
        match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => ChaCha8Rng::from_entropy(),
        }
    }

    /// Draws a random hand from the deck
    fn draw_random_hand(&self, deck: &[Card], hand_size: usize, rng: &mut ChaCha8Rng) -> Vec<Card> {
        let mut deck_copy = deck.to_vec();
        deck_copy.shuffle(rng);
        deck_copy.into_iter().take(hand_size).collect()
    }

    /// Calculates statistics from collected scores
    fn calculate_statistics(&self, mut scores: Vec<u64>, num_runs: usize) -> SimulationResult {
        scores.sort_unstable();

        let mean_score = scores.iter().sum::<u64>() as f64 / num_runs as f64;
        let median_score = self.percentile(&scores, 0.5);
        let min_score = *scores.first().unwrap_or(&0);
        let max_score = *scores.last().unwrap_or(&0);

        SimulationResult {
            num_runs,
            mean_score,
            median_score,
            min_score,
            max_score,
            percentile_25: self.percentile(&scores, 0.25),
            percentile_75: self.percentile(&scores, 0.75),
            percentile_95: self.percentile(&scores, 0.95),
        }
    }

    /// Calculates a percentile from sorted scores
    fn percentile(&self, sorted_scores: &[u64], p: f64) -> u64 {
        if sorted_scores.is_empty() {
            return 0;
        }

        let index = ((sorted_scores.len() as f64 - 1.0) * p) as usize;
        sorted_scores[index]
    }
}

/// Creates a standard 52-card deck
pub fn create_standard_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);
    let ranks = [
        Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
        Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
        Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
    ];
    let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

    for suit in suits {
        for rank in ranks {
            deck.push(Card::new(rank, suit));
        }
    }

    deck
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::scoring::ScoreCalculator;

    #[test]
    fn test_standard_deck_creation() {
        let deck = create_standard_deck();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn test_simulation_with_seed() {
        let deck = create_standard_deck();
        let calculator = ScoreCalculator::new(vec![]);
        let solver = Solver::new(calculator);
        let simulator = Simulator::new(solver);

        let config = SimulationConfig {
            deck,
            hand_size: 5,
            num_runs: 10,
            seed: Some(42),
        };

        let result = simulator.simulate(config);
        assert_eq!(result.num_runs, 10);
        assert!(result.mean_score > 0.0);
    }
}
