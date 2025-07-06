fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir("src/")
        .compile(&["proto/virtfs.proto"], &["proto"])
        .unwrap();
}
