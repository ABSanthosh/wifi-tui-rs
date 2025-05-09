#[derive(Debug, Clone)]
pub struct Network {
    pub ssid: String,
    pub signal: u8,
    pub secured: bool,
    pub connected: bool,
}
