/// Core logic of SVC
// === Module declarations ===
pub mod cli;
pub mod commands;
pub mod config;
pub mod document;
pub mod fs_utils;
pub mod utils;
pub mod vcs;

pub fn handle_command(config: Config) {
    let result = match config.command {
        Command::Init(values) => handle_init(&values.name),
        Command::AddSection(values) => handle_add_section(&values.name),
        Command::Diff(values) => {}
        Command::Build(values) => {}
    };
}

pub fn handle_init(name: &str) {
    println!("Initializing {name} project");
    match fs_utils::init_project_structure(name) {
        Ok(_) => println!("Successfully initialized empty project"),
        Err(e) => println!("An Error occurred while initializing the project {e}"),
    }
}

pub fn handle_add_section(name: &str) {
    println!("Adding new section ({name})");
    match fs_utils::init_section_structure(name) {
        Ok(_) => println!("Successfully initialized new section"),
        Err(e) => println!("Unable to create section: {e}"),
    }
}

// === Re-Exports (public API) ===
// ClI <-> Core Mapping
pub use cli::*;

// Core Commands and Values
pub use commands::{AddSectionValues, BuildValues, Command, DiffValues, InitValues};

// Config
pub use config::Config;

// Document-Logic
pub use document::*;

// FS-Utilities
pub use fs_utils::*;

// Version Control
pub use vcs::*;
