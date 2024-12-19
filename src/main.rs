use network_manager::NetworkManager;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let network_manager = NetworkManager::new();

    let devices = network_manager.get_devices()?;
    let mut wifi_networks = Vec::new();

    for device in devices {
        if let Some(wifi) = device.as_wifi_device() {
            let access_points = wifi.get_access_points()?;
            wifi_networks.extend(access_points);
        }
    }

    println!("Available Wi-Fi Networks:");
    for ap in wifi_networks {
        let ssid_slice = ap.ssid();
        match ssid_slice.as_str() {
            Ok(ssid) => println!("{}", ssid),
            Err(_) => println!("(Hidden Network)"),
        }
    }

    Ok(())
}

