use std::{collections::HashMap, process::Command};

#[derive(Clone, Debug)]
pub struct Network {
    pub ssid: String,
    pub signal: String,
    pub security: String,
    pub rate: String,
    pub bars: String,
    pub is_connected: bool
}

impl Network {

    fn from_nmcli_line(line: &str, active_ssid: &str) -> Option<Network> {
        let fields: Vec<&str> = line.split(':').collect();

        if fields.len() < 5 || fields[0].is_empty() {
            return None;
        }

        Some(Network {
            ssid: fields[0].to_string(),
            signal: fields[1].to_string(),
            security: fields[2].to_string(),
            rate: fields[3].to_string(),
            bars: fields[4].to_string(),
            is_connected: fields[0] == active_ssid,
        })
    }

}

pub fn get_networks() -> Result<Vec<Network>, String> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "SSID,SIGNAL,SECURITY,RATE,BARS", "device", "wifi", "list"])
        .output()
        .expect("Failed to execute nmcli");

    let active_output = Command::new("nmcli")
        .args(["-t", "-f", "SSID,ACTIVE", "device", "wifi"])
        .output()
        .expect("Failed to execute nmcli for active connection");

    let active_ssid = String::from_utf8_lossy(&active_output.stdout)
    .lines()
    .find_map(|line| {
        if line.ends_with(":yes") {
            line.strip_suffix(":yes").map(|ssid| ssid.to_string())
        } else {
            None
        }
    })
    .unwrap_or_else(|| {
        eprintln!("No active SSID found.");
        String::new()
    });

    let mut networks_map = HashMap::new();

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if let Some(network) = Network::from_nmcli_line(line, &active_ssid) {
            networks_map
                .entry(network.ssid.clone())
                .and_modify(|existing: &mut Network| {
                    if network.signal > existing.signal {
                        *existing = network.clone();
                    }
                })
                .or_insert(network);
        }
    }
    Ok(networks_map.into_values().collect())
}

