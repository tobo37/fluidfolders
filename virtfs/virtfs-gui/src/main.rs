use tauri::{Builder, command, generate_context};
use virtfs_common::{virtfs_client::VirtfsClient, LayoutName};

#[command]
async fn list_layouts() -> Vec<String> {
    VirtfsClient::connect_uds("/tmp/virtfs.sock")
        .await
        .unwrap()
        .list_layouts(())
        .await
        .unwrap()
        .into_inner()
        .names
}

#[command]
async fn set_layout(name: String) {
    let _ = VirtfsClient::connect_uds("/tmp/virtfs.sock")
        .await
        .unwrap()
        .set_layout(LayoutName { name })
        .await;
}

fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![list_layouts, set_layout])
        .run(generate_context!())
        .expect("error running tauri application");
}
