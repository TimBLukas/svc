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
