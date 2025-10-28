//! UI rendering logic for the TUI

use crate::tui::app::{App, SelectedTab};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

/// Main draw function - renders the entire UI
pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header/tabs
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Input bar
        ])
        .split(f.area());

    draw_tabs(f, app, chunks[0]);
    draw_content(f, app, chunks[1]);
    draw_input(f, app, chunks[2]);
}

/// Draws the tab bar at the top
fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["Solver", "Simulator", "Config"];
    let selected = match app.selected_tab {
        SelectedTab::Solver => 0,
        SelectedTab::Simulator => 1,
        SelectedTab::Config => 2,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Jimbo"))
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

/// Draws the main content area based on selected tab
fn draw_content(f: &mut Frame, app: &App, area: Rect) {
    match app.selected_tab {
        SelectedTab::Solver => draw_solver_tab(f, area),
        SelectedTab::Simulator => draw_simulator_tab(f, area),
        SelectedTab::Config => draw_config_tab(f, area),
    }
}

/// Draws the solver tab content
fn draw_solver_tab(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Hand Solver")
        .borders(Borders::ALL);

    let text = vec![
        Line::from("Enter your hand to find the optimal play"),
        Line::from(""),
        Line::from(Span::styled(
            "Coming soon: Interactive hand builder",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

/// Draws the simulator tab content
fn draw_simulator_tab(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Monte Carlo Simulator")
        .borders(Borders::ALL);

    let text = vec![
        Line::from("Run simulations to test your joker builds"),
        Line::from(""),
        Line::from(Span::styled(
            "Coming soon: Build configuration and simulation runs",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

/// Draws the config tab content
fn draw_config_tab(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Configuration")
        .borders(Borders::ALL);

    let text = vec![
        Line::from("Manage deck configurations and game states"),
        Line::from(""),
        Line::from(Span::styled(
            "Coming soon: Load/save configurations",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

/// Draws the input bar at the bottom
fn draw_input(f: &mut Frame, app: &App, area: Rect) {
    let input = Paragraph::new(Text::from(app.input.as_str()))
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input (q to quit, Tab to switch tabs)"),
        );

    f.render_widget(input, area);
}
