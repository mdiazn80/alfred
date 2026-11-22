//! Application state and event handling.

use std::time::Duration;

use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// How long to wait for an input event before redrawing.
const TICK_RATE: Duration = Duration::from_millis(250);

/// Top-level application state.
#[derive(Debug, Default)]
pub struct App {
    /// Whether the main loop should keep running.
    running: bool,
    /// Example piece of state rendered by the UI.
    counter: i64,
}

impl App {
    /// Creates a new application in its initial state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs the main loop until the user quits.
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| crate::ui::render(&self, frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Current counter value.
    pub fn counter(&self) -> i64 {
        self.counter
    }

    /// Polls for terminal events and dispatches them.
    fn handle_events(&mut self) -> Result<()> {
        if event::poll(TICK_RATE)? {
            match event::read()? {
                // Only react to presses (Windows also emits release events).
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key(key),
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles a single key press.
    pub fn on_key(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c' | 'C')) => self.quit(),
            (_, KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('+')) => self.increment(),
            (_, KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('-')) => self.decrement(),
            (_, KeyCode::Char('r')) => self.counter = 0,
            _ => {}
        }
    }

    fn increment(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    fn decrement(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn press(app: &mut App, code: KeyCode) {
        app.on_key(KeyEvent::new(code, KeyModifiers::NONE));
    }

    #[test]
    fn increments_and_decrements() {
        let mut app = App::new();
        press(&mut app, KeyCode::Right);
        press(&mut app, KeyCode::Char('+'));
        press(&mut app, KeyCode::Left);
        assert_eq!(app.counter(), 1);
        press(&mut app, KeyCode::Char('r'));
        assert_eq!(app.counter(), 0);
    }

    #[test]
    fn quits_on_q_esc_and_ctrl_c() {
        for key in [
            KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
        ] {
            let mut app = App {
                running: true,
                ..App::default()
            };
            app.on_key(key);
            assert!(!app.running);
        }
    }
}
