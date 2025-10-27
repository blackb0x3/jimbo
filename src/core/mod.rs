//! Core game logic for Balatro simulation and solving
//!
//! This module contains the fundamental data structures and algorithms
//! for representing and evaluating Balatro game states.

pub mod card;
pub mod hand;
pub mod joker;
pub mod scoring;
pub mod simulator;
pub mod solver;

// Re-export commonly used types
pub use card::{Card, Enhancement, Edition, Rank, Suit};
pub use hand::{Hand, HandType};
pub use joker::Joker;
pub use scoring::{ScoreCalculator, ScoreResult};
pub use simulator::{create_standard_deck, SimulationConfig, SimulationResult, Simulator};
pub use solver::Solver;
