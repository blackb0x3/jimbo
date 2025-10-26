//! Configuration file handling
//!
//! This module handles loading, saving, and validating configuration files
//! for decks, game states, and presets.

pub mod deck;
pub mod game_state;

// Re-export commonly used types
pub use deck::DeckConfig;
pub use game_state::GameState;
