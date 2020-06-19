use crate::tile::Block;
use dbus::nonblock::stdintf::org_freedesktop_dbus::Properties;
use dbus::nonblock::{Proxy, SyncConnection};
use futures::{FutureExt, Stream};
use std::sync::Arc;
use std::time::Duration;

pub fn hostname_stream(
    connection: Arc<SyncConnection>,
) -> impl Stream<Item = Result<Block, Box<dyn std::error::Error + Send + Sync>>> {
    let proxy = Proxy::new(
        "org.freedesktop.hostname1",
        "/org/freedesktop/hostname1",
        Duration::from_secs(5),
        connection,
    );
    let reply = proxy.get("org.freedesktop.hostname1", "Hostname");
    async {
        let hostname: String = reply.await?;
        let block = Block {
            full_text: hostname.into(),
            name: "hostname".into(),
            ..Default::default()
        };
        Ok(block)
    }
    .into_stream()
}
