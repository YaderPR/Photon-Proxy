use anyhow::{bail, Result};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::proxy_engine::upstream_relay::UpstreamClient;
use crate::proxy_engine::linux_tproxy::create_marked_tcp_socket;

pub struct HttpProxyClient;

impl UpstreamClient for HttpProxyClient {
    async fn connect(
        &self,
        target: &SocketAddr,
        server: &SocketAddr,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Result<TcpStream> {
        let socket = create_marked_tcp_socket(server)?;
        let mut stream = socket.connect(*server).await?;

        // Format CONNECT request
        let mut request = format!(
            "CONNECT {}:{} HTTP/1.1\r\n\
            Host: {}:{}\r\n\
            User-Agent: PhotonProxy/0.1\r\n",
            target.ip(), target.port(),
            target.ip(), target.port()
        );

        if let (Some(u), Some(p)) = (username, password) {
            let auth_str = format!("{}:{}", u, p);
            let b64 = BASE64.encode(auth_str);
            request.push_str(&format!("Proxy-Authorization: Basic {}\r\n", b64));
        }

        request.push_str("\r\n");

        stream.write_all(request.as_bytes()).await?;

        // Read the proxy server's HTTP response
        // Typically something like: "HTTP/1.1 200 Connection established\r\n\r\n"
        let mut buffer = [0u8; 1024];
        let mut response_str = String::new();

        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                bail!("HTTP Proxy disconnected prematurely");
            }
            
            response_str.push_str(&String::from_utf8_lossy(&buffer[..n]));

            // End of headers signal
            if response_str.contains("\r\n\r\n") {
                break;
            }
        }

        let status_line = response_str.lines().next().unwrap_or("");
        if !status_line.contains(" 200") {
            bail!("HTTP Proxy CONNECT requested failed: {}", status_line);
        }

        Ok(stream)
    }
}
