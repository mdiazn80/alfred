//! Alfred — a terminal user interface built with [Ratatui](https://ratatui.rs).

mod app;
mod ui;

use app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // `ratatui::run` initializes the terminal (raw mode + alternate screen),
    // installs a panic hook and restores the terminal when the closure returns.
    ratatui::run(|terminal| App::new().run(terminal))
}
