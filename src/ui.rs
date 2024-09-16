// src/ui.rs

use crossterm::event::KeyCode;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use strum::IntoEnumIterator;

use crate::{
    app::{CommandType, InputMode},
    App,
};

pub fn draw<B: Backend>(f: &mut Frame, app: &App) {
    // Create the main layout
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3), // For the title
                Constraint::Min(2),    // For the menu or input
                Constraint::Length(3), // For status messages
            ]
            .as_ref(),
        )
        .split(size);

    draw_title::<B>(f, chunks[0], app);
    match app.input_mode {
        InputMode::Normal => draw_menu::<B>(f, chunks[1], app),
        InputMode::Editing => {
            match app.current_command {
                CommandType::RunTests => draw_argument_input::<B>(f, chunks[1], app, "Run Tests"),
                CommandType::GenerateTestFromService => {
                    draw_argument_input::<B>(f, chunks[1], app, "Path to Service")
                }
                CommandType::SetRootDirectory => draw_input::<B>(f, chunks[1], app),

                // Handle other commands
                _ => {}
            }
        }
    }

    draw_status_bar::<B>(f, chunks[2], app);
}

fn draw_title<B: Backend>(f: &mut Frame, area: Rect, app: &App) {
    let title = Paragraph::new(format!("Dotnet Tools - Root: {}", app.root_directory))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn draw_menu<B: Backend>(f: &mut Frame, area: Rect, _app: &App) {
    let menu_items: Vec<ListItem> = CommandType::iter()
        .filter(|command| command.is_visible())
        .map(|command| {
            let key_string = match command.key() {
                KeyCode::Char(c) => c.to_string(),
                KeyCode::F(n) => format!("F{}", n),
                KeyCode::Esc => "Esc".to_string(),
                KeyCode::Enter => "Enter".to_string(),
                _ => "".to_string(),
            };
            ListItem::new(format!("{}. {}", key_string, command.display_name()))
        })
        .collect();

    let menu =
        List::new(menu_items).block(Block::default().title("Main Menu").borders(Borders::ALL));

    f.render_widget(menu, area);
}

fn draw_input<B: Backend>(f: &mut Frame, area: Rect, app: &App) {
    let input = Paragraph::new::<Text>(app.input.clone().into())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .title("Set Root Directory")
                .borders(Borders::ALL),
        );
    f.render_widget(input, area);
    f.set_cursor(
        // Put cursor past the end of the input text
        area.x + app.input.len() as u16 + 1,
        area.y + 1,
    );
}

fn draw_status_bar<B: Backend>(f: &mut Frame, area: Rect, app: &App) {
    let status = Paragraph::new::<Text>(app.status_message.clone().into())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status, area);
}

fn draw_argument_input<B: Backend>(f: &mut Frame, area: Rect, app: &App, title: &str) {
    let input = Paragraph::new::<Text>(app.input.clone().into())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .title(format!("{} - Additional Arguments", title))
                .borders(Borders::ALL),
        );
    f.render_widget(input, area);
    f.set_cursor(area.x + app.input.len() as u16 + 1, area.y + 1);
}
