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
        Command::AddSection(values) => Ok(()),
        Command::Diff(values) => Ok(()),
        Command::Build(values) => Ok(()),
    };
}

pub fn handle_init(name: &str) -> std::result::Result<(), std::io::Error> {
    println!("Initializing {name} project");
    fs_utils::init_project_structure(name)?;
    Ok(())
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
