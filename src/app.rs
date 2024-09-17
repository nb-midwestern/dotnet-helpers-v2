// src/app.rs

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::Backend, Terminal};
use std::{env, io, path::PathBuf, ptr::null};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::handlers;
use crate::ui;

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum CommandType {
    GenerateTestFromService,
    SetOutputFile,
    GetInputFile,
    UpdateDependencies,
    CleanSolution,
    BuildProject,
    RunTests,
    SetRootDirectory,
    Quit,
    None,
}

impl CommandType {
    pub fn key(&self) -> KeyCode {
        match self {
            CommandType::UpdateDependencies => KeyCode::Char('1'),
            CommandType::CleanSolution => KeyCode::Char('2'),
            CommandType::BuildProject => KeyCode::Char('3'),
            CommandType::RunTests => KeyCode::Char('4'),
            CommandType::SetRootDirectory => KeyCode::Char('5'),
            CommandType::Quit => KeyCode::Char('q'),
            CommandType::None => KeyCode::Null,
            CommandType::GenerateTestFromService => KeyCode::Char('t'),
            CommandType::SetOutputFile => KeyCode::Char('o'),
            CommandType::GetInputFile => KeyCode::Null,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            CommandType::UpdateDependencies => "Update Dependencies",
            CommandType::CleanSolution => "Clean Solution",
            CommandType::BuildProject => "Build Project",
            CommandType::RunTests => "Run Tests",
            CommandType::SetRootDirectory => "Set Root Directory",
            CommandType::Quit => "Quit",
            CommandType::None => " ",
            CommandType::GenerateTestFromService => "Generate test from Service",
            CommandType::SetOutputFile => "Set Output File",
            CommandType::GetInputFile => "",
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            CommandType::None => false,
            CommandType::GetInputFile => false,
            _ => true,
        }
    }

    pub fn requires_additional_args(&self) -> bool {
        match self {
            CommandType::GenerateTestFromService => true,
            CommandType::RunTests => true,
            CommandType::SetRootDirectory => true,
            CommandType::SetOutputFile => true,
            CommandType::GetInputFile => true,
            _ => false,
        }
    }

    pub fn from_keycode(keycode: KeyCode) -> Option<Self> {
        CommandType::iter().find(|&command| command.key() == keycode)
    }
}

pub struct App {
    pub should_quit: bool,
    pub root_directory: String,
    pub input_mode: InputMode,
    pub input: String,
    pub status_message: String,
    pub current_command: CommandType,
    pub additional_args: String,
    pub output_file: PathBuf,
    pub input_file: PathBuf,
}

impl App {
    pub fn new() -> Self {
        let root_directory = env::current_dir()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|e| {
                // Log the error or update the status message
                eprintln!("Failed to get current directory: {}", e);
                String::from(".")
            });

        App {
            should_quit: false,
            root_directory: root_directory,
            input_mode: InputMode::Normal,
            input: String::new(),
            status_message: String::from(""),
            current_command: CommandType::None,
            additional_args: String::new(),
            output_file: PathBuf::new(),
            input_file: PathBuf::new(),
        }
    }

    pub fn set_root_directory(&mut self, dir: String) {
        let path = PathBuf::from(&dir);
        if path.is_dir() {
            self.root_directory = path.to_string_lossy().into_owned();
            self.status_message = format!("Root directory set to {}.", self.root_directory);
        } else {
            self.status_message = String::from("Invalid directory!");
        }
    }

    pub fn set_input_file(&mut self, path: PathBuf) {
        if path.is_file() {
            self.input_file = path;
            self.status_message =
                format!("Input File set to {}.", self.input_file.to_string_lossy())
        } else {
            self.status_message = format!("Invalid File {}", path.to_string_lossy())
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    // Enable raw mode to capture input events
    enable_raw_mode()?;
    execute!(io::stdout(), EnableMouseCapture)?;

    loop {
        // Draw the UI
        terminal.draw(|f| {
            ui::draw::<B>(f, app);
        })?;

        // Handle input events
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let CEvent::Key(key) = event::read()? {
                handlers::handle_key_events(app, key);
            }
        }

        // Check if the app should quit
        if app.should_quit {
            disable_raw_mode()?;
            execute!(io::stdout(), DisableMouseCapture)?;
            break;
        }
    }

    Ok(())
}
