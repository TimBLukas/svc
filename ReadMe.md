# SVC

> Study Version Control - A CLI Tool for managing versions of study reports

![ Rust](https://img.shields.io/badge/Rust-2021-orange.svg?logo=rust)

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)

![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)

![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg)

## Purpose

SVC is a command line interface (CLI) tool inspired by Git, but specifically designed for manageing study-related reports (e.g. seminar papers, projects reports, theses).

Instead of managing code, SVC manages document sections (Markdown first, Word/DOCX planned later) and allows you to:

- Create new projects with a fixed folder structure
- Write and organize sections individually.
- Commit changes with version history (hash-based)
- Compare differences between versions (diff).
- Configure global/project-specific settings (font, title page, references).
- Build final reports (PDF + DOCX) by merging sections

## Features (planned)

- **Project Initialization**:
  - `svc init` creates a new project with predefined folder structure (`sections/`, `.svc/`, `output/`).
  - Project-specific config is stored in `.svc/config.json`.

- Section Management
  - `svc add-section` creates a new section file (Markdown by default).
  - Sections are stored in `sections/` with metadata (title, type, order).

- Version Control
  - `svc commit -m "message"` creates a snapshot of all sections.
  - Commits identified via hashes.
  - `svc diff <hash1> <hash2>` shows differences in paragraphs, lists, tables, etc.

- Configurations
  - Global config: `~/.svcconfig`.
  - Project config: `.svc/config.json`
  - Config options include default font, title page template, wheter to include ToC, bibliography, list of figures.

- Build/Export
  - `svc build` merges all sections into a single report
  - Generates both **PDF** and **DOCX**.
  - Automatically adds title page, table of contents, bibliography.

- Extensibility
  - Modular architecture using Rust traits.
  - Future support for Word parsing (`docx-rs`) and LaTeX export.
  - Potential team collaboration via remote repositories (future).

## Project structure

### Workspace layout

svc/
├── Cargo.toml # Workspace definition
├── svc-core/ # Core library crate
│ ├── Cargo.toml
│ └── src/
│ ├── lib.rs
│ ├── cli/ # CLI structures
│ ├── commands/ # Subcommands (init, commit, build, ...)
│ ├── config/ # Config handling
│ ├── vcs/ # Versioning logic
│ ├── document/ # Section + builder
│ └── utils.rs
├── svc-cli/ # CLI binary crate
│ ├── Cargo.toml
│ └── src/main.rs
└── tests/ # Integration tests

### Project Layout (after `svc init`)

my_report/
├── .svc/ # Internal metadata
│ ├── config.json # Project config
│ ├── commits/ # Commit snapshots
│ └── index.json # Mapping sections ↔ commits
├── sections/ # User-written sections
│ ├── 01_intro.md
│ ├── 02_methodology.md
│ └── ...
└── output/ # Generated reports
├── report.pdf
└── report.docx

## Getting Started

### Prerequisites

- Rust (edition 2021)
- Optioanl: Pandoc for advanced PDF/DOCX conversion

### Build from Source

```bash
# Clone the repository
git clone https://github.com/TimBLukas/svc.git
cd svc

# Build the entire workspace
cargo build

# Run CLI
cargo run -p svc-cli -- --help
```

### Example Usage

```bash
# Initialize a new project
svc init my_report

cd my_report

# Add a new section
svc add-section --title "Introduction" --type paragraph --file intro.md
```

## Technology

- **Language** : Rust
- **Core Crates**:
  - `clap` -> CLI parsing
  - `serde` + `serde_json` -> Config handling
  - `sha2` -> Commit hashes
  - `pulldown-cmark` -> Mardown parsing
  - `thiserror` -> Error handling

- **Planned**:
  - `docx-rs` -> DOCX parsing & generation
  - `pandoc`(Optional) -> PDF/DOCX rendering
