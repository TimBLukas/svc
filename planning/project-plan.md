# Projektplan

## Möglich Struktur

svc/ # Root des Workspaces
├── Cargo.toml # Workspace-Definition
├── svc-cli/ # Binary-Crate (CLI)
│ ├── Cargo.toml
│ └── src/
│ └── main.rs # Einstiegspunkt, nur CLI-Parsing & Calls an Library
├── svc-core/ # Library-Crate (Core-Logik)
│ ├── Cargo.toml
│ └── src/
│ ├── lib.rs # Entry Point der Library
│ ├── cli/ # CLI-Unterstützung (nur Struktur/Argumente)
│ │ ├── mod.rs
│ │ └── commands/
│ │ ├── mod.rs
│ │ ├── init.rs
│ │ ├── add_section.rs
│ │ ├── commit.rs
│ │ ├── build.rs
│ │ └── diff.rs
│ ├── config/
│ │ ├── mod.rs
│ │ └── models.rs
│ ├── vcs/
│ │ ├── mod.rs
│ │ ├── commit.rs
│ │ └── diff.rs
│ ├── document/
│ │ ├── mod.rs
│ │ ├── section.rs
│ │ └── builder.rs
│ └── utils.rs # Hilfsfunktionen
└── tests/ # Workspace-weite Integrationstests

- Dabei sind `scv-core` und `scv-cli` eigene Workspaces, um die cli und die restliche Funktionalität zu trennen

## Crates und Techniken

### CLI & Argumentenparsing

- Crates: `clap`
- Flags vs. interaktive Eingaben
- Subcommands (`init`, `add-section`, `commit`, `build`, `diff`)

### Datei- und Ordneroperationen

- `std::fs` für Dateioperationen
- Path-Hanlding: `Path`/`PathBuf`
- Umgang mit versteckten Ordnern (`.svc/`)
- Cross Plattform Pfade (Windows/Linux/MacOS)

### Konfiguration

- Globale Config (`~/.svcconfig`) vs. Projektbezogene Config (`.svc/config.json`)
- Serde + serde_json oder toml zum Einlesen/Schreiben von Configs

### Versionierung und Commits

- Hashes: `sha2` oder `blake3d` zur eindeutigen Commit-ID
- Snapshots-Mechanismus: gesamte Sektionen oder nur geänderte Elemente?
- Diffing auf Ebene der Rust-Structs -> Eigene Implementierung für Paragraphs, Lists, Tables

### Markdown & Word Parsing

- Markdown: `pulldown-cmark` oder `comrak`
- Word/Docx: `docx.rs`
- Idee: Gemeinsame Rust-Structs für alle Formate

### Build/Export

- PDF/Word-Erzeugung: Crates oder externe Tools (pandoc via Command)
- Titelblatt, Inhaltsverzeichnis, Literaturverzeichnis
- Optional: Stylesheets / Fonts konfigurieren
