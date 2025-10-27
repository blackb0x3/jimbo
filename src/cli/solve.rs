//! Solve command implementation
//!
//! This module implements the `solve` command which finds the optimal
//! play from a given hand.

use crate::config::DeckConfig;
use crate::core::{Card, Joker, Rank, ScoreCalculator, Solver, Suit};
use anyhow::{Context, Result};
use clap::Args;

/// Arguments for the solve command
#[derive(Debug, Args)]
pub struct SolveArgs {
    /// Your current hand (space-separated, e.g., "AH KH QH JH 10H")
    #[arg(long, required = true)]
    hand: String,

    /// Path to deck configuration file (JSON)
    #[arg(long)]
    deck: Option<String>,

    /// Comma-separated list of jokers (e.g., "Joker,GreedyJoker")
    #[arg(long, value_delimiter = ',')]
    jokers: Vec<String>,

    /// Required score to beat the blind
    #[arg(long)]
    blind_score: Option<u64>,

    /// Optional seed for reproducible results
    #[arg(long)]
    seed: Option<u64>,

    /// Output format: pretty (default), json, compact
    #[arg(long, default_value = "pretty")]
    output: OutputFormat,

    /// Show top N alternative plays (default: 3)
    #[arg(long, default_value = "3")]
    show_alternatives: usize,
}

/// Output format for the solve command
#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Pretty,
    Json,
    Compact,
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "pretty" => Ok(OutputFormat::Pretty),
            "json" => Ok(OutputFormat::Json),
            "compact" => Ok(OutputFormat::Compact),
            _ => anyhow::bail!("Invalid output format: {}. Use 'pretty', 'json', or 'compact'", s),
        }
    }
}

/// Runs the solve command
pub fn run(args: SolveArgs) -> Result<()> {
    // Parse the hand
    let cards = parse_hand(&args.hand)?;

    if cards.is_empty() {
        anyhow::bail!("Hand cannot be empty");
    }

    // Load deck config if provided
    if let Some(deck_path) = &args.deck {
        let _deck_config = DeckConfig::from_file(deck_path)
            .with_context(|| format!("Failed to load deck config from {}", deck_path))?;
        // TODO: Use deck config to modify cards based on enhancements/editions
    }

    // Parse jokers
    let jokers = parse_jokers(&args.jokers)?;

    // Create score calculator and solver
    let calculator = ScoreCalculator::new(jokers);
    let solver = Solver::new(calculator);

    // Solve for the best play
    let result = solver.solve(&cards);

    // Display results based on output format
    match args.output {
        OutputFormat::Pretty => display_pretty(&result, &args),
        OutputFormat::Json => display_json(&result)?,
        OutputFormat::Compact => display_compact(&result),
    }

    Ok(())
}

/// Parses a hand string into a vector of cards
fn parse_hand(hand_str: &str) -> Result<Vec<Card>> {
    let tokens: Vec<&str> = hand_str.split_whitespace().collect();
    let mut cards = Vec::new();

    for token in tokens {
        let card = parse_card(token)?;
        cards.push(card);
    }

    Ok(cards)
}

/// Parses a single card string (e.g., "AH", "10D", "KS")
fn parse_card(card_str: &str) -> Result<Card> {
    if card_str.len() < 2 {
        anyhow::bail!("Invalid card format: {}", card_str);
    }

    // Split into rank and suit
    let (rank_str, suit_str) = if card_str.starts_with("10") {
        ("10", &card_str[2..])
    } else {
        (&card_str[..card_str.len() - 1], &card_str[card_str.len() - 1..])
    };

    let rank = parse_rank(rank_str)?;
    let suit = parse_suit(suit_str)?;

    Ok(Card::new(rank, suit))
}

/// Parses a rank string
fn parse_rank(s: &str) -> Result<Rank> {
    match s {
        "2" => Ok(Rank::Two),
        "3" => Ok(Rank::Three),
        "4" => Ok(Rank::Four),
        "5" => Ok(Rank::Five),
        "6" => Ok(Rank::Six),
        "7" => Ok(Rank::Seven),
        "8" => Ok(Rank::Eight),
        "9" => Ok(Rank::Nine),
        "10" => Ok(Rank::Ten),
        "J" => Ok(Rank::Jack),
        "Q" => Ok(Rank::Queen),
        "K" => Ok(Rank::King),
        "A" => Ok(Rank::Ace),
        _ => anyhow::bail!("Invalid rank: {}", s),
    }
}

/// Parses a suit string
fn parse_suit(s: &str) -> Result<Suit> {
    match s.to_uppercase().as_str() {
        "H" => Ok(Suit::Hearts),
        "D" => Ok(Suit::Diamonds),
        "C" => Ok(Suit::Clubs),
        "S" => Ok(Suit::Spades),
        _ => anyhow::bail!("Invalid suit: {}", s),
    }
}

/// Parses joker names into Joker objects
fn parse_jokers(_joker_names: &[String]) -> Result<Vec<Joker>> {
    // TODO: Implement proper joker name parsing
    // For now, return empty vector
    Ok(Vec::new())
}

/// Displays results in pretty format
fn display_pretty(result: &crate::core::solver::SolverResult, args: &SolveArgs) {
    if let Some(score_result) = &result.best_score {
        println!("🃏 Best Play:");
        println!("  Hand Type: {:?}", score_result.hand_type);
        println!("  Cards: {}", format_cards(&result.best_hand.cards));
        println!("  Score: {}", score_result.score);
        println!("  Chips: {} × Mult: {} = {}",
            score_result.chips,
            score_result.mult,
            score_result.score
        );

        if let Some(blind_score) = args.blind_score {
            if score_result.score >= blind_score {
                println!("  ✅ Beats blind (required: {})", blind_score);
            } else {
                println!("  ❌ Does not beat blind (required: {}, short by: {})",
                    blind_score,
                    blind_score - score_result.score
                );
            }
        }

        // Show alternatives
        if args.show_alternatives > 0 && !result.alternatives.is_empty() {
            println!("\n📋 Alternative Plays:");
            for (i, (hand, score)) in result.alternatives.iter().take(args.show_alternatives).enumerate() {
                println!("  {}. {:?} - {} - Score: {}",
                    i + 1,
                    score.hand_type,
                    format_cards(&hand.cards),
                    score.score
                );
            }
        }
    } else {
        println!("No valid plays found");
    }
}

/// Displays results in JSON format
fn display_json(result: &crate::core::solver::SolverResult) -> Result<()> {
    let json = serde_json::json!({
        "best_hand": {
            "cards": result.best_hand.cards.len(),
            "score": result.best_score.as_ref().map(|s| s.score),
            "hand_type": result.best_score.as_ref().map(|s| format!("{:?}", s.hand_type)),
            "chips": result.best_score.as_ref().map(|s| s.chips),
            "mult": result.best_score.as_ref().map(|s| s.mult),
        },
        "alternatives": result.alternatives.iter().map(|(_, score)| {
            serde_json::json!({
                "score": score.score,
                "hand_type": format!("{:?}", score.hand_type),
            })
        }).collect::<Vec<_>>(),
    });

    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}

/// Displays results in compact format
fn display_compact(result: &crate::core::solver::SolverResult) {
    if let Some(score_result) = &result.best_score {
        println!("{:?} | {} | Score: {}",
            score_result.hand_type,
            format_cards(&result.best_hand.cards),
            score_result.score
        );
    } else {
        println!("No valid plays");
    }
}

/// Formats cards for display
fn format_cards(cards: &[Card]) -> String {
    cards.iter().map(|c| format_card(c)).collect::<Vec<_>>().join(" ")
}

/// Formats a single card for display
fn format_card(card: &Card) -> String {
    let rank = match card.rank {
        Rank::Two => "2",
        Rank::Three => "3",
        Rank::Four => "4",
        Rank::Five => "5",
        Rank::Six => "6",
        Rank::Seven => "7",
        Rank::Eight => "8",
        Rank::Nine => "9",
        Rank::Ten => "10",
        Rank::Jack => "J",
        Rank::Queen => "Q",
        Rank::King => "K",
        Rank::Ace => "A",
    };

    let suit = match card.suit {
        Suit::Hearts => "♥",
        Suit::Diamonds => "♦",
        Suit::Clubs => "♣",
        Suit::Spades => "♠",
    };

    format!("{}{}", rank, suit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let card = parse_card("AH").unwrap();
        assert_eq!(card.rank, Rank::Ace);
        assert_eq!(card.suit, Suit::Hearts);

        let card = parse_card("10D").unwrap();
        assert_eq!(card.rank, Rank::Ten);
        assert_eq!(card.suit, Suit::Diamonds);
    }

    #[test]
    fn test_parse_hand() {
        let cards = parse_hand("AH KH QH JH 10H").unwrap();
        assert_eq!(cards.len(), 5);
        assert_eq!(cards[0].rank, Rank::Ace);
        assert_eq!(cards[4].rank, Rank::Ten);
    }

    #[test]
    fn test_invalid_card() {
        assert!(parse_card("XX").is_err());
        assert!(parse_card("1H").is_err());
    }
}
