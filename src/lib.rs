//! Jimbo - Your personal Balatro strategist
//!
//! This library provides the core game logic, solver, and simulator for Balatro.
//! It can be used as a standalone library or through the CLI/TUI interfaces.

// Public modules
pub mod cli;
pub mod config;
pub mod core;
pub mod tui;

// Re-export commonly used types at the crate root for convenience
pub use crate::core::*;
