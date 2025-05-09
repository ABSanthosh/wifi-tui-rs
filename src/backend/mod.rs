use crate::models::Network;
use anyhow::Result;

#[async_trait::async_trait]
pub trait WifiBackend {
    async fn scan_networks(&self) -> Result<Vec<Network>>;
    async fn connect(&self, ssid: &str, password: Option<&str>) -> Result<()>;
    async fn disconnect(&self, ssid: &str) -> Result<()>;
    async fn known_networks(&self) -> Result<Vec<Network>>;
}
