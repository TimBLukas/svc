/// Core logic of SVC
// === Module declarations ===
pub mod cli;
pub mod commands;
pub mod config;
pub mod document;
pub mod fs_utils;
pub mod utils;
pub mod vcs;

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
