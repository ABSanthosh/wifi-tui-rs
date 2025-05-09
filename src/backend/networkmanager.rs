use anyhow::Result;
use zbus::zvariant::OwnedValue;
use zbus::Connection;

pub async fn print_devices() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = zbus::Proxy::new(
        &connection,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
    )
    .await?;

    let devices: Vec<OwnedValue> = proxy.call("GetDevices", &()).await?;
    println!("Devices: {:?}", devices);

    Ok(())
}
