use crate::proxy_engine::linux_tproxy::create_marked_tcp_socket;
use crate::proxy_engine::upstream_relay::UpstreamClient;
use anyhow::{bail, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

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
            target.ip(),
            target.port(),
            target.ip(),
            target.port()
        );

        if let (Some(u), Some(p)) = (username, password) {
            let auth_str = format!("{}:{}", u, p);
            let b64 = BASE64.encode(auth_str);
            request.push_str(&format!("Proxy-Authorization: Basic {}\r\n", b64));
        }

        request.push_str("\r\n");

        stream.write_all(request.as_bytes()).await?;

        // Read the proxy server's HTTP response byte by byte to avoid over-reading TLS data!
        // If we read a large chunk, we might accidentally consume the first TLS packet
        // that arrives right after the 200 OK, breaking the zero-copy splice.
        let mut response_str = String::new();
        let mut single_byte = [0u8; 1];

        loop {
            let n = stream.read(&mut single_byte).await?;
            if n == 0 {
                bail!("HTTP Proxy disconnected prematurely");
            }

            response_str.push(single_byte[0] as char);

            if response_str.ends_with("\r\n\r\n") {
                break;
            }

            // Safety cap to prevent infinite memory leak
            if response_str.len() > 8192 {
                bail!("HTTP Proxy response too long or missing end of headers");
            }
        }

        let status_line = response_str.lines().next().unwrap_or("");
        if !status_line.contains(" 200") {
            bail!("HTTP Proxy CONNECT requested failed: {}", status_line);
        }

        Ok(stream)
    }
}
