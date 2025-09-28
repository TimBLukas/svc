# SVC

## To Do's (Long Term)

- [ ] Grundgerüst und CLI
  - CLI mit `svc init`, `svc add-section`, `scv commit`, `svc build`
  - Feste Ordnerstrukturen mit `init` erzeugen (`/sections`, `/meta`, `/commits`).

- [ ] Abschnitssverwaltung
  - `add-section`: neue Dateien anlegen (Markdown),
  - Flags für Titel, Typ, Reihenfolge

- [ ] Versionierung
  - Commit-System mit Hashes
  - Speichern von Snapshots + Metadaten (Commit-Message, Datum).
  - `svc-diff <hash1> <hash2>` für Unterschiede (Paragraph/Listen/Tabellen-Structs).

- [ ] Konfiguration
  - Globale Config (`~/.svcconfig`) + projektbezogene Config (`.scv/config.json`)
  - Optionen: Schriftart, Titelblatt, Verzeichnisse (Inhalt/Literatur/Abbildungen).

- [ ] Build-Pipeline
  - Markdown zu PDF (via `pandoc`-Crate).
  - Titelblatt + automatische Verzeichnisse einbinden.
  - Parallel Docx Export.

- [ ] Erweiterbarkeit
  - Abstrakte Schnittstellen (Traits) für Input/Output Formate
  - Vorbereitung für Team-Kollaboration (Remote-Repos als zukünftige Möglichkeit)

## To Do's (shortterm)

- [ ] Projekt-Setup
  - [ ] Rust Projekt anlegen
  - [ ] CLI-Struktur mit `clap` oder `structopt` anlegen (`scv init`, `scv add-section` ... nur leere Commands)

- [ ] `scv init` implementieren
  - [ ] Ordnerstruktur anlegen

- [ ] Konfiguration
  - global (Für alle Projekte als Standard)
  - lokal (Für ein spezifisches Projekt -> Überschreit globale Config)

- [ ] `add-section` rudimentär umsetzen
  - Markdown Datei in `sections/` mit Metadaten (Titel, Typ) erzeugen

- [ ] Einfache Commit Struktur
  - Hash generieren (z.B. Sha356 des Snapshots).
  - Commit-Metadaten in `.svc/commits.json` speichern.

## Dateiinhalte:

- commands.rs
  Command Enum mit allen möglichen Befehlen
  Passende Structs für Argumentenwerde
  Eventuell execute(&self) Implementationen, wenn die Commands selbst ausführbar sein sollen

- cli.rs
  Schnittstelle zum CLI (Mapping zwischen args::Command und svc_core::Command)
  parse_args() oder map_cli_to_core
  Nur Übersetzung von user Input zu Core

- config.rs
  Config Struct das einen Command enthält (zentrale Datenstruktur)
  Optionale Einstellungen (Standardschriftart)
  Funktionen, um Config aus CLI/Dateien zu laden

- document.rs
  Alles für das Dokumentenmodell
  -> Abschnitte (Paragraphen, Tabellen, Abbildungen (als Structs und oder Enums))
  -> Zusammenführen der Sections in einem Bericht

- fs_utils.rs
  - Low Level Dateisystem Helfer
    -> Projektstruktur anlegen
    -> Dateien/Ordner lesen und schreiben
    -> ggf. Hashing für Commits vorbereits (evtl aber auch in vcs.rs)
    !Nur Dateioperationen

  - lib.rs
    -> Einstiegspunkt des Library Crates
    -> Exportiert dei wichtigen Module (pub mod commands, pub mod config, ...)

  - utils.rs
    -> Allgemeine Helferfunktionen, die nicht in eine spezifisches Modul passen
    -> z.B. String formatierung, Hash berechnungen, kleine Parser

  - vcs.rs
    -> Kern der Versionskontrolle
    -> Commit erstellen, (aktuellen Stand snapshotten -> Hash)
    -> Diffs zwischen zwei Versionen berechnen
    -> Commit Verlauf verwalten (History)
