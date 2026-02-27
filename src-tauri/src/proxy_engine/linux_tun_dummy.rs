use crate::proxy_engine::ProxyAdapter;
use anyhow::Result;

pub struct TunAdapter {
    interface_name: String,
}

impl TunAdapter {
    pub fn new(interface_name: &str) -> Self {
        Self {
            interface_name: interface_name.to_string(),
        }
    }
}

impl ProxyAdapter for TunAdapter {
    async fn start(&mut self, _app_handle: tauri::AppHandle) -> Result<()> {
        log::info!("Starting TunAdapter placeholder");
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        log::warn!("TunAdapter::stop is a placeholder and not implemented.");
        Ok(())
    }
}
