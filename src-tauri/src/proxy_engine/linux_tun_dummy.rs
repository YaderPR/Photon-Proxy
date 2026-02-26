use anyhow::Result;
use crate::proxy_engine::ProxyAdapter;

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
    async fn start(&mut self) -> Result<()> {
        log::warn!("TunAdapter::start is a placeholder and not implemented.");
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        log::warn!("TunAdapter::stop is a placeholder and not implemented.");
        Ok(())
    }
}
