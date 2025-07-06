# Projektübersicht FluidFolders

Dieses Repository enthält einen Rust-Workspace unter `virtfs` mit drei Crates:

- **virtfs-common**: gRPC-Definitionen und generierter Code.
- **virtfsd**: FUSE-Daemon, stellt den Virtfs-Dienst bereit.
- **virtfs-gui**: Tauri-Oberfläche, kommuniziert über `/tmp/virtfs.sock` mit `virtfsd`.

Zum Bauen und Testen siehe `README.md`.
