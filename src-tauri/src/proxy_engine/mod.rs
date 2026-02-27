pub mod linux_tproxy;
pub mod linux_tun_dummy;
pub mod system_rules;
pub mod upstream_relay;
pub mod zero_copy;

use anyhow::Result;

/// Core trait representing a proxy routing strategy.
/// Allows swapping between TPROXY, TUN, or Windows equivalent adapters.
pub trait ProxyAdapter: Send + Sync {
    /// Initialize and start the proxy adapter (e.g. bind sockets, set up rules)
    #[allow(async_fn_in_trait)]
    async fn start(&mut self, app_handle: tauri::AppHandle) -> Result<()>;

    /// Stop the proxy adapter and clean up any system rules or routines
    #[allow(async_fn_in_trait)]
    async fn stop(&mut self) -> Result<()>;
}
