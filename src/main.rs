use std::collections::HashMap;
use std::process::Command;

pub struct WiFiNetwork {
    pub ssid: String,
    pub strength: i32,
    pub security: String,
    pub connected: bool,
    pub uuid: String,
}

struct NetworkManager {
    networks: HashMap<String, WiFiNetwork>,
}

impl NetworkManager {
    pub fn new() -> Self {
        NetworkManager {
            networks: HashMap::new(),
        }
    }

    // Get the list of visible Wi-Fi networks
    pub async fn get(&mut self) {
        // CLI equivalent: nmcli -t -f SSID,SIGNAL,SECURITY,ACTIVE,SSID-HEX device wifi list
        let output = Command::new("nmcli")
            .args(&[
                "-t",
                "-f",
                "SSID,SIGNAL,SECURITY,ACTIVE,SSID-HEX",
                "device",
                "wifi",
                "list",
            ])
            .output()
            .expect("Failed to execute nmcli");

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            for line in result.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 3 && !parts[0].is_empty() {
                    self.networks.insert(
                        parts[0].to_string().trim().to_string(),
                        WiFiNetwork {
                            ssid: parts[0].to_string().trim().to_string(),
                            strength: parts[1].parse().unwrap_or(0),
                            security: parts[2].to_string(),
                            connected: parts[3] == "yes",
                            uuid: parts[4].to_string(),
                        },
                    );
                }
            }
        } else {
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    // // Connect to a Wi-Fi network
    pub async fn connect(&self, ssid: &str, password: &str) {
        // CLI equivalent: nmcli dev wifi connect <SSID> password <PASSWORD>
        let output = Command::new("nmcli")
            .args(&["dev", "wifi", "connect", ssid, "password", password])
            .output()
            .expect("Failed to execute nmcli");

        if output.status.success() {
            println!("Connected to {}", ssid);
        } else {
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    // // Disconnect from a Wi-Fi network
    // pub async fn disconnect(&self, ssid: &str) {}
}

#[tokio::main]
async fn main() {
    let ssid = "Recurrence";
    let password = "asdfghjkl";

    // let ssid = "MP-312";
    // let password = "tuntunmausi";

    let mut nm = NetworkManager::new();
    nm.get().await;
    for (ssid, network) in &nm.networks {
        println!(
            "SSID: |{}|, Strength: {}, Security: {}, Connected: {}, UUID: {}",
            ssid, network.strength, network.security, network.connected, network.uuid
        );
    }
    nm.connect(ssid, password).await;
}
