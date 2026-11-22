//! Rendering logic. Keep this module free of state mutation.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Paragraph};

use crate::app::App;

/// Draws the whole application into the given frame.
pub fn render(app: &App, frame: &mut Frame) {
    let [main, footer] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(Line::from(" Alfred ").bold().cyan().centered());

    let body = Paragraph::new(vec![
        Line::from("Welcome to Alfred, a Ratatui application."),
        Line::from(""),
        Line::from(vec![
            "Counter: ".into(),
            app.counter().to_string().bold().yellow(),
        ]),
    ])
    .block(block)
    .centered();

    let help = Line::from("←/h decrement · →/l increment · r reset · q/Esc quit")
        .style(Style::new().dim())
        .centered();

    frame.render_widget(body, main);
    frame.render_widget(help, footer);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    #[test]
    fn renders_title_and_counter() {
        let app = App::new();
        let mut terminal = Terminal::new(TestBackend::new(60, 8)).unwrap();
        terminal.draw(|frame| render(&app, frame)).unwrap();

        let content: String = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect();
        assert!(content.contains("Alfred"));
        assert!(content.contains("Counter: 0"));
    }
}
