use dbus::nonblock::Proxy;
use dbus_tokio::connection;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We can't do much until we have a D-Bus connection so just do it synchronously
    let (resource, conn) = connection::new_session_sync()?;

    // Now start listening on our D-Bus connection
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    Ok(())
}
