use crate::proxy_engine::linux_tproxy::LinuxTproxyAdapter;
use crate::proxy_engine::upstream_relay::UpstreamProtocol;
use crate::proxy_engine::ProxyAdapter;
use std::net::SocketAddr;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

pub struct ProxyState {
    pub engine: Arc<Mutex<Option<LinuxTproxyAdapter>>>,
}

impl Default for ProxyState {
    fn default() -> Self {
        Self {
            engine: Arc::new(Mutex::new(None)),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ProxyConfig {
    pub local_port: u16,
    pub upstream_type: String, // "direct", "socks5", "http"
    pub upstream_host: Option<String>,
    pub upstream_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[tauri::command]
pub async fn start_proxy(
    config: ProxyConfig,
    state: State<'_, ProxyState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let mut lock = state.engine.lock().await;
    if lock.is_some() {
        return Err("Proxy is already running".into());
    }

    let upstream = match config.upstream_type.as_str() {
        "direct" => UpstreamProtocol::Direct,
        "socks5" | "http" => {
            let host = config.upstream_host.as_deref().unwrap_or("127.0.0.1");
            let port = config.upstream_port.unwrap_or(1080);
            let addr_str = format!("{}:{}", host, port);

            let addr: SocketAddr = match addr_str.parse() {
                Ok(a) => a,
                Err(_) => return Err(format!("Invalid upstream address: {}", addr_str)),
            };

            if config.upstream_type == "socks5" {
                UpstreamProtocol::Socks5 {
                    server: addr,
                    username: config.username.clone(),
                    password: config.password.clone(),
                }
            } else {
                UpstreamProtocol::HttpProxy {
                    server: addr,
                    username: config.username.clone(),
                    password: config.password.clone(),
                }
            }
        }
        _ => return Err(format!("Unknown upstream type: {}", config.upstream_type)),
    };

    let mut engine = LinuxTproxyAdapter::new(config.local_port, upstream, app_handle.clone());
    if let Err(e) = engine.start(app_handle).await {
        return Err(format!("Failed to start proxy engine: {}", e));
    }

    *lock = Some(engine);
    Ok("Proxy started successfully".into())
}

#[tauri::command]
pub async fn stop_proxy(state: State<'_, ProxyState>) -> Result<String, String> {
    let mut lock = state.engine.lock().await;
    if let Some(mut engine) = lock.take() {
        if let Err(e) = engine.stop().await {
            return Err(format!("Failed to stop cleanly: {}", e));
        }
        Ok("Proxy stopped successfully".into())
    } else {
        Err("Proxy is not running".into())
    }
}

#[tauri::command]
pub async fn get_proxy_status(state: State<'_, ProxyState>) -> Result<bool, String> {
    let lock = state.engine.lock().await;
    Ok(lock.is_some())
}
