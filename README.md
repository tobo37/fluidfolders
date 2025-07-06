# FluidFolders

Diese Anleitung beschreibt, wie du das Projekt lokal einrichtest, baust und testest.

## Voraussetzungen

* Eine aktuelle [Rust](https://www.rust-lang.org/tools/install) Installation (empfohlen via `rustup`)
* Je nach Plattform weitere Abhängigkeiten für Tauri, z.B. Node.js

## Repository klonen

```bash
git clone <REPO-URL>
cd fluidfolders
```

## Build

Das Rust-Workspace befindet sich im Unterordner `virtfs`.
Baue alle Komponenten mit:

```bash
cd virtfs
cargo build --workspace
```

Für ein Release-Build:

```bash
cargo build --release --workspace
```

## Tests ausführen

Um die vorhandenen Tests zu starten (aktuell gibt es noch keine),
verwende ebenfalls im `virtfs` Verzeichnis:

```bash
cargo test --workspace
```

## Programme starten

Die einzelnen Binaries kannst du wie folgt aufrufen:

```bash
# FUSE-Daemon
cargo run -p virtfsd

# Graphische Benutzeroberfläche
cargo run -p virtfs-gui
```

Damit wird „virtfsd“ im Hintergrund gestartet und die GUI verbindet
sich über den Unix-Socket `/tmp/virtfs.sock` mit dem Dienst.
