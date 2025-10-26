# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Jimbo is a command-line solver and simulator for the game Balatro. It analyzes hands, jokers, consumables, and blind conditions to find optimal plays and simulate scoring outcomes. Written in Rust for performance.

## Build & Development Commands

```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run the CLI
cargo run -- <command> [options]

# Run tests
cargo test

# Run specific test
cargo test <test_name>

# Run with verbose output
cargo test -- --nocapture

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Architecture Overview

### Module Structure

The project is split into a **binary crate** (CLI entry point) and **library crate** (core logic):

- **`src/main.rs`**: Binary entry point. Parses CLI arguments using `clap` and dispatches to appropriate command handlers
- **`src/lib.rs`**: Library root. Re-exports core types for convenience
- **`src/cli/`**: Command handlers for `solve`, `simulate`, `config` subcommands
- **`src/core/`**: Core game logic including:
  - Card/hand representations
  - Scoring engine (chips, mult calculation)
  - Joker effects and modifiers
  - Hand evaluation (pair, flush, straight, etc.)
  - Solver algorithm
- **`src/config/`**: Configuration file handling (deck configs, game state serialization)
- **`src/tui/`**: Terminal UI using `ratatui` and `crossterm`

### Key Commands

1. **`solve`**: Analyzes current hand state and finds the optimal card combination to play
   - Takes: hand, jokers, deck config, blind info, seed
   - Outputs: best play with score breakdown

2. **`simulate`**: Runs Monte Carlo simulations over many random hands
   - Takes: deck config, joker build, number of runs
   - Outputs: statistics (mean, median, percentiles)

3. **`tui`**: Interactive terminal interface for building and testing configurations

4. **`config`**: Manages JSON configuration files for decks, game states, and presets

### Data Flow

```
CLI Args → Command Handler → Core Game Logic → Output Formatter
                ↓
           Config Files (JSON)
```

For `solve`:
1. Parse input (hand cards, jokers, blind)
2. Generate all possible hand combinations (C(n,k) for valid poker hands)
3. For each combination, calculate score with joker effects applied
4. Return highest-scoring combination

For `simulate`:
1. Initialize deck and joker state
2. Draw random hands N times (with optional seed for reproducibility)
3. For each hand, run solve logic
4. Aggregate statistics

### Randomness & Seeds

- Uses `rand_chacha` for deterministic RNG when seed is provided
- Seed affects: hand draws in simulation, joker randomness (if applicable)
- Ensures reproducible results for testing and sharing builds

### Scoring System

Balatro scoring: `Score = Chips × Mult`

- Base hand values (pair, two pair, straight, etc.)
- Card enhancements (gold, steel, glass, etc.)
- Card editions (foil, holographic, polychrome)
- Joker effects (additive/multiplicative chip/mult bonuses, retriggering, etc.)
- Voucher effects (global modifiers)

The solver must apply all effects in the correct order per Balatro's game rules.

## Development Guidelines

### Adding New Jokers

1. Define joker enum variant in `core/joker.rs`
2. Implement effect logic in scoring calculator
3. Add serialization support (name mapping)
4. Add tests for edge cases

### Adding New Commands

1. Create module in `src/cli/<command>.rs`
2. Define `Args` struct with `clap` derives
3. Implement `run(args: Args) -> anyhow::Result<()>`
4. Register in `src/main.rs` enum and match statement
5. Export from `src/cli/mod.rs`

### Configuration Files

Use JSON for deck configs and game states:
- Cards: `{"rank": "A", "suit": "Hearts"}`
- Enhancements/editions: map card ID to enhancement name
- Jokers: list of joker names (snake_case or Title_Case)

Validate configs on load and provide clear error messages.

## Dependencies

- **clap**: CLI argument parsing with derive macros
- **ratatui + crossterm**: Terminal UI framework
- **serde + serde_json**: Config serialization
- **anyhow**: Error handling with context
- **thiserror**: Custom error types
- **rand + rand_chacha**: Seeded RNG for reproducibility

## Testing Strategy

- Unit tests for scoring logic (core calculations)
- Integration tests for command handlers (CLI → output)
- Property tests for hand evaluation (all combinations valid)
- Snapshot tests for specific joker interactions

Use seeds in tests to ensure deterministic behavior.
