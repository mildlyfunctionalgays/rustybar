use crate::config::IwdConfig;
use crate::tile::Block;
use crate::tiles::TileResult;
use dbus::arg::{PropMap, RefArg};
use dbus::nonblock::stdintf::org_freedesktop_dbus::ObjectManager;
use dbus::nonblock::{Proxy, SyncConnection};
use dbus::Path;
use futures::stream::Stream;
use futures_async_stream::try_stream;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

fn get_interface_path(
    objects: &HashMap<Path<'static>, HashMap<String, PropMap>>,
    interface_name: &str,
) -> Option<Path<'static>> {
    for (path, property_groups) in objects.iter() {
        for (dbus_interface, properties) in property_groups.iter() {
            if dbus_interface.as_str() == "net.connman.iwd.Device" {
                if let Some(name) = properties.get("Name") {
                    if Some(interface_name) == name.as_str() {
                        return Some(path.clone());
                    }
                }
            }
        }
    }
    None
}

pub fn iwd_stream(
    connection: Arc<SyncConnection>,
    config: IwdConfig,
) -> impl Stream<Item = TileResult> {
    let root_proxy = Proxy::new("net.connman.iwd", "/", Duration::from_secs(5), connection);
    #[try_stream]
    async move {
        loop {
            let managed_objects = root_proxy.get_managed_objects().await?;
            let interface_path = get_interface_path(&managed_objects, &config.interface)
                .ok_or("Couldn't find interface")?;
            let station_path = managed_objects
                .get(&interface_path)
                .and_then(|object| object.get("net.connman.iwd.Station"))
                .and_then(|interface| interface.get("ConnectedNetwork"))
                .and_then(|network| network.as_str())
                .and_then(|s| Path::from_slice(s).ok());
            let text = if let Some(path) = station_path {
                let network_name = managed_objects
                    .get(&path)
                    .and_then(|object| object.get("net.connman.iwd.Network"))
                    .and_then(|network| network.get("Name"))
                    .and_then(|name| name.as_str())
                    .ok_or("Unable to get network name")?;
                format!("{}: {}", &config.interface, network_name)
            } else {
                format!("{}: disconnected", &config.interface)
            };
            yield Block {
                full_text: text.into(),
                name: config.interface.clone(),
                ..Default::default()
            };
        }
    }
}
