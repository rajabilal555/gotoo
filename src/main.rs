mod editor;
mod errors;
mod project;

extern crate clap;

use clap::{Arg, ArgAction, Command};
use editor::Editor;
use inquire::{Select, Text};
use project::Project;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::vec;

use crate::errors::CmdError;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Gotoo")
        .version("0.1")
        .author("Bilal Pervez")
        .about("Manage and open project directories in your favourite editors")
        .arg(
            Arg::new("open")
                .short('o')
                .action(ArgAction::SetTrue)
                .help("Navigate to and open a project directory")
                .exclusive(true),
        )
        .arg(
            Arg::new("add")
                .short('a')
                .action(ArgAction::SetTrue)
                .help("Add a new project to the data source")
                .exclusive(true),
        )
        .get_matches();

    let mut action: Option<&str> = None;
    if matches.get_flag("open") {
        action = Some("open");
    } else if matches.get_flag("add") {
        action = Some("add");
    }

    if action == None {
        println!("Welcome to Project Manager!");
        let paction = Select::new("What do you want to do?", vec!["open", "add"]).prompt();

        match paction {
            Ok(paction) => {
                print!("You selected {}!", paction);
                action = Some(paction);
            }
            Err(_) => println!("There was an error, please try again"),
        }
    }

    let mut projects: Vec<Project> = match load_projects() {
        Ok(existing_projects) => existing_projects,
        Err(_) => Vec::new(), // If file doesn't exist or has an error, start with an empty Vec
    };

    if action == Some("test") {
        println!("Test values hehe {}", action.unwrap());
    }

    if action == Some("open") {
        let project = Select::new("Enter Project Name:", projects).prompt()?;

        let editor = Select::new(
            "Select an Editor:",
            vec![Editor::VsCode, Editor::Neovim, Editor::Vim],
        )
        .prompt()?;

        // Load the project directory from your data store (e.g., JSON or database)
        let project_directory = project.directory;

        // Check if the directory exists
        if !Path::new(&project_directory).exists() {
            return Err(Box::new(CmdError(
                "Project directory does not exist".into(),
            )));
        }

        // Launch the specified editor in the project directory
        launch_editor(editor, project_directory);
    } else if action == Some("add") {
        let project_name = Text::new("Enter a name for your project").prompt()?;
        let project_directory = Text::new("Select a directory").prompt()?;

        let new_project = Project {
            name: project_name.to_string(),
            directory: project_directory.to_string(),
        };
        projects.push(new_project);

        if let Err(e) = save_projects(&projects) {
            return Err(Box::new(CmdError(
                format!("Error writing projects to file: {}", e).into(),
            )));
        } else {
            println!("Project added successfully.");
        }
    }

    Ok(())
}

fn launch_editor(editor: Editor, directory: String) {
    match editor.as_str() {
        "vscode" => {
            std::process::Command::new("cmd")
                .args(&["/c", "code", directory.as_str()])
                .spawn()
                .expect("Failed to start VSCode");
        }
        "nvim" | "vim" => {
            std::process::Command::new("cmd")
                .args(&["/C", "start", "", editor.as_str(), directory.as_str()])
                .spawn()
                .expect("Failed to start the editor");
        }
        _ => println!("Unsupported editor"),
    }
}

fn load_projects() -> io::Result<Vec<Project>> {
    let data = fs::read_to_string("projects.json")?;
    let projects: Vec<Project> = serde_json::from_str(&data)?;
    Ok(projects)
}

fn save_projects(projects: &Vec<Project>) -> Result<(), std::io::Error> {
    let serialized = serde_json::to_string(&projects)?;
    // Open the data store file in append mode and write the serialized project data.
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("projects.json")?;
    writeln!(file, "{}", serialized)?;

    Ok(())
}
