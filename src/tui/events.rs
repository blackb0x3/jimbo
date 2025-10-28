//! Event handling for keyboard and terminal events

use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent};
use std::time::Duration;

/// Poll timeout duration in milliseconds
const POLL_TIMEOUT_MS: u64 = 100;

/// Polls for keyboard events with a timeout
///
/// Returns `Ok(Some(event))` if an event is available,
/// `Ok(None)` if the timeout elapsed with no event,
/// or an error if polling failed.
pub fn poll_event() -> Result<Option<KeyEvent>> {
    if event::poll(Duration::from_millis(POLL_TIMEOUT_MS))? {
        if let Event::Key(key_event) = event::read()? {
            return Ok(Some(key_event));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poll_timeout() {
        // This test verifies that polling completes within the timeout period
        // In non-interactive environments, polling may fail or return immediately
        let start = std::time::Instant::now();
        let _result = poll_event();
        let elapsed = start.elapsed();

        // Should not take significantly longer than the timeout
        // (may return immediately in non-interactive environments)
        assert!(elapsed < Duration::from_millis(POLL_TIMEOUT_MS + 100));
    }
}
