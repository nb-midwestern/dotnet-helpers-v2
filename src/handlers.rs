// src/handlers.rs

use crate::{
    app::{CommandType, InputMode},
    commands, App,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(app: &mut App, key_event: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => {
            if key_event.code == KeyCode::Char('c')
                && key_event.modifiers.contains(KeyModifiers::CONTROL)
            {
                app.should_quit = true;
                return;
            }
            if let Some(command) = CommandType::from_keycode(key_event.code) {
                match command {
                    CommandType::SetRootDirectory => {
                        app.input_mode = InputMode::Editing;
                        app.input.clear();
                        app.current_command = command;
                        app.status_message = String::from("Enter the root directory path:");
                    }
                    _ if command.requires_additional_args() => {
                        app.input_mode = InputMode::Editing;
                        app.input.clear();
                        app.current_command = command;
                        app.status_message = format!(
                            "Enter additional arguments for '{}':",
                            command.display_name()
                        );
                    }
                    _ => {
                        execute_command(command, app);
                    }
                }
            }
        }
        InputMode::Editing => match key_event.code {
            KeyCode::Enter => {
                let args = app.input.clone();
                app.additional_args = args;
                execute_command(app.current_command, app);
                app.input_mode = InputMode::Normal;
                app.input.clear();
                app.current_command = CommandType::None;
                app.additional_args.clear();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
                app.input.clear();
                app.status_message = String::from("Cancelled input.");
                app.current_command = CommandType::None;
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            _ => {}
        },
    }
}

fn execute_command(command: CommandType, app: &mut App) {
    match command {
        CommandType::UpdateDependencies => commands::update_dependencies(app),
        CommandType::CleanSolution => commands::clean_solution(app),
        CommandType::BuildProject => commands::build_project(app),
        CommandType::RunTests => commands::run_tests(app),
        CommandType::SetRootDirectory => commands::set_root_directory(app),
        CommandType::GenerateTestFromService => commands::generate_test_from_service(app),
        CommandType::Quit => app.should_quit = true,
        _ => {}
    }
}
