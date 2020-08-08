use crate::tile::Block;
use crate::tiles::TileResult;
use dbus::nonblock::stdintf::org_freedesktop_dbus::Properties;
use dbus::nonblock::{Proxy, SyncConnection};
use futures::stream::Stream;
use futures_async_stream::try_stream;
use std::time::Duration;

pub fn hostname_stream(connection: &SyncConnection) -> impl Stream<Item = TileResult> {
    let proxy = Proxy::new(
        "org.freedesktop.hostname1",
        "/org/freedesktop/hostname1",
        Duration::from_secs(5),
        connection,
    );
    let reply = proxy.get("org.freedesktop.hostname1", "Hostname");
    #[try_stream]
    async move {
        let hostname: String = reply.await?;
        yield Block {
            full_text: hostname.into(),
            name: "hostname".into(),
            ..Default::default()
        };
    }
}
