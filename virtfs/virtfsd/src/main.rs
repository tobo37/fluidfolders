use fuser::{Filesystem, MountOption, ReplyDirectory, Request};
use std::path::PathBuf;
use tokio::{signal, task};
use tokio_stream::wrappers::UnixListenerStream;
use virtfs_common::virtfs::virtfs_server::{Virtfs, VirtfsServer};
use virtfs_common::virtfs::{LayoutList, LayoutName};

struct Vfs;

impl Filesystem for Vfs {
    fn readdir(&mut self, _req: &Request<'_>, _ino: u64, _fh: u64, _offset: i64, reply: ReplyDirectory) {
        reply.ok();
    }
}

async fn switch_layout(_name: &str) {
    // placeholder for switching logic
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mountpoint = PathBuf::from("/tmp/virtfs");
    let fs = Vfs;

    let fuse_task = task::spawn_blocking(move || {
        fuser::mount2(
            fs,
            &mountpoint,
            &[MountOption::FSName("virtfs".to_string()), MountOption::RO],
        )
        .unwrap();
    });

    struct RpcSvc;
    #[tonic::async_trait]
    impl Virtfs for RpcSvc {
        async fn list_layouts(&self, _req: tonic::Request<()>) -> tonic::Result<tonic::Response<LayoutList>> {
            Ok(tonic::Response::new(LayoutList { names: vec![] }))
        }
        async fn set_layout(&self, req: tonic::Request<LayoutName>) -> tonic::Result<tonic::Response<()>> {
            switch_layout(&req.into_inner().name).await;
            Ok(tonic::Response::new(()))
        }
    }

    let uds = tokio::net::UnixListener::bind("/tmp/virtfs.sock")?;
    let grpc = tonic::transport::Server::builder()
        .add_service(VirtfsServer::new(RpcSvc))
        .serve_with_incoming(UnixListenerStream::new(uds));

    tokio::select! {
        _ = fuse_task => {},
        _ = grpc => {},
        _ = signal::ctrl_c() => {}
    }

    Ok(())
}
