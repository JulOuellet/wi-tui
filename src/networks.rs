use std::{collections::HashMap, process::Command};

#[derive(Clone, Debug)]
pub struct Network {
    pub ssid: String,
    pub signal: String,
    pub security: String,
    pub rate: String,
    pub bars: String
}

pub fn get_networks() -> Vec<Network> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "SSID,SIGNAL,SECURITY,RATE,BARS", "device", "wifi", "list"])
        .output()
        .expect("Failed to execute nmcli");

    let mut networks_map = HashMap::new();

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split(':').collect();

            if fields.len() >= 5 && !fields[0].is_empty() {
                Some(
                        Network {
                            ssid:     fields[0].to_string(),
                            signal:   fields[1].to_string(),
                            security: fields[2].to_string(),
                            rate:     fields[3].to_string(),
                            bars:     fields[4].to_string()
                        }
                )
            } else {
                None
            }
        })
        .for_each(|network| {
            networks_map
                .entry(network.ssid.clone())
                .and_modify(|existing: &mut Network| {
                    if network.signal.parse::<i32>().unwrap_or(0) > 
                       existing.signal.parse::<i32>().unwrap_or(0) {
                        *existing = network.clone();
                    }
                })
                .or_insert(network);
        });

        networks_map.into_values().collect()
}
