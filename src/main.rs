// src/main.rs

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

mod app;
mod commands;
mod csharp;
mod handlers;
mod ui;
use app::App;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Run app
    let res = app::run_app(&mut terminal, &mut app);

    // Restore terminal
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
