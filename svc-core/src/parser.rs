// Submodules
pub mod ast;
pub mod markdown;
pub mod word;

// Re-Exports
pub use ast::*;

// Convenience functions

// pub fn parse_file(path: &Path) -> Result<Document, create::error::SvcError> {
//     match path.extension().and_then(|s| s.to_str()) {
//         Some("md") => markdown::parse_file(path),
//         Some("docx") => word::parse_file(path),
//         _ => Err(SvcError::UnsupportedFormat),
//     }
// }
