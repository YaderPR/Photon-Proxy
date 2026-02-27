pub mod socks5;
pub mod http_proxy;

use std::net::SocketAddr;
use tokio::net::TcpStream;
use anyhow::Result;

#[derive(Clone, Debug)]
pub enum UpstreamProtocol {
    Direct,
    Socks5 {
        server: SocketAddr,
        username: Option<String>,
        password: Option<String>,
    },
    HttpProxy {
        server: SocketAddr,
        username: Option<String>,
        password: Option<String>,
    }
}

pub trait UpstreamClient {
    /// Connects to the target destination through the specific proxy protocol.
    /// The returned `TcpStream` is the fully negotiated, ready-to-use tunnel.
    async fn connect(
        &self, 
        target: &SocketAddr, 
        server: &SocketAddr, 
        username: Option<&str>, 
        password: Option<&str>
    ) -> Result<TcpStream>;
}
