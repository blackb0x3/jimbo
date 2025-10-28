//! Application state management for the TUI

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Application state
pub struct App {
    /// Whether the application should quit
    should_quit: bool,
    /// Current input buffer for user text entry
    pub input: String,
    /// Currently selected tab/section
    pub selected_tab: SelectedTab,
}

/// Represents which tab is currently selected in the UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedTab {
    /// Hand solver tab
    Solver,
    /// Simulator tab
    Simulator,
    /// Configuration tab
    Config,
}

impl App {
    /// Creates a new application instance with default state
    pub fn new() -> Self {
        Self {
            should_quit: false,
            input: String::new(),
            selected_tab: SelectedTab::Solver,
        }
    }

    /// Handles keyboard events and returns false if the app should quit
    pub fn handle_event(&mut self, event: KeyEvent) -> bool {
        match event.code {
            // Quit on Ctrl+C or 'q'
            KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            // Tab navigation
            KeyCode::Tab => {
                self.next_tab();
            }
            KeyCode::BackTab => {
                self.previous_tab();
            }
            // Input handling
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                self.handle_submit();
            }
            _ => {}
        }

        !self.should_quit
    }

    /// Moves to the next tab
    fn next_tab(&mut self) {
        self.selected_tab = match self.selected_tab {
            SelectedTab::Solver => SelectedTab::Simulator,
            SelectedTab::Simulator => SelectedTab::Config,
            SelectedTab::Config => SelectedTab::Solver,
        };
    }

    /// Moves to the previous tab
    fn previous_tab(&mut self) {
        self.selected_tab = match self.selected_tab {
            SelectedTab::Solver => SelectedTab::Config,
            SelectedTab::Simulator => SelectedTab::Solver,
            SelectedTab::Config => SelectedTab::Simulator,
        };
    }

    /// Handles submission of the current input
    fn handle_submit(&mut self) {
        // TODO: Process the input based on the current tab
        self.input.clear();
    }

    /// Returns whether the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_navigation() {
        let mut app = App::new();
        assert_eq!(app.selected_tab, SelectedTab::Solver);

        app.next_tab();
        assert_eq!(app.selected_tab, SelectedTab::Simulator);

        app.next_tab();
        assert_eq!(app.selected_tab, SelectedTab::Config);

        app.next_tab();
        assert_eq!(app.selected_tab, SelectedTab::Solver);
    }

    #[test]
    fn test_input_handling() {
        let mut app = App::new();

        let event = KeyEvent::from(KeyCode::Char('a'));
        app.handle_event(event);
        assert_eq!(app.input, "a");

        let event = KeyEvent::from(KeyCode::Backspace);
        app.handle_event(event);
        assert_eq!(app.input, "");
    }

    #[test]
    fn test_quit() {
        let mut app = App::new();
        assert!(!app.should_quit());

        let event = KeyEvent::from(KeyCode::Char('q'));
        let should_continue = app.handle_event(event);
        assert!(!should_continue);
        assert!(app.should_quit());
    }
}
