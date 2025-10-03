pub mod builder;
pub mod cli;
/// Core logic of SVC
// === Module declarations ===
pub mod commands;
pub mod config;
pub mod errors;
pub mod fs_utils;
pub mod parser;
pub mod storage;

pub fn handle_command(config: Config) {
    let result = match config.command {
        Commands::Init(init_command) => init_command.execute_initialization(),
        Commands::AddSection(add_section_command) => add_section_command.execute_add_section(),
        Commands::Diff(values) => {}
        Commands::Build(values) => {}
        Commands::Commit(values) => {}
        Commands::Status => {}
    };
}

// === Re-Exports (public API) ===
// ClI <-> Core Mapping
pub use commands::*;

// Config
pub use config::Config;

// FS-Utilities
pub use fs_utils::*;
