pub mod generate_test_from_service;

use std::process::Command;

use crate::{app::CommandType, App};

fn execute_command(command: &mut Command, app: &mut App, action: &str) {
    app.status_message = format!("{}...", action);
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                app.status_message = format!("{} completed successfully.", action);
            } else {
                app.status_message = format!(
                    "Error during {}:\n{}",
                    action,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            app.status_message = format!("Failed to execute '{}': {}", action, e);
        }
    }
}

pub fn update_dependencies(app: &mut App) {
    let root_dir = app.root_directory.clone();
    let mut command = Command::new("dotnet");
    command.arg("restore").current_dir(root_dir);
    execute_command(&mut command, app, "Updating dependencies");
}

pub fn clean_solution(app: &mut App) {
    let root_dir = app.root_directory.clone();
    let mut command = Command::new("dotnet");
    command.arg("clean").current_dir(root_dir);
    execute_command(&mut command, app, "Cleaning solution");
}

pub fn build_project(app: &mut App) {
    let root_dir = app.root_directory.clone();
    let mut command = Command::new("dotnet");
    command.arg("build").current_dir(root_dir);
    execute_command(&mut command, app, "Building project");
}

pub fn run_tests(app: &mut App) {
    let root_dir = app.root_directory.clone();
    let additional_args = app.additional_args.split_whitespace();

    let mut command = Command::new("dotnet");
    command
        .arg("test")
        .args(additional_args)
        .current_dir(root_dir);
    execute_command(&mut command, app, "Running tests");
}

pub fn set_root_directory(app: &mut App) {
    let new_dir = app.additional_args.clone();
    app.set_root_directory(new_dir);
}

pub fn generate_test_from_service(app: &mut App) {
    let service = app.additional_args.clone();
    // app.current_command = CommandType::GetOutputDir;
}
