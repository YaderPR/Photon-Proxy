use anyhow::{bail, Result};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::proxy_engine::upstream_relay::UpstreamClient;
use crate::proxy_engine::linux_tproxy::create_marked_tcp_socket;

pub struct Socks5Client;

impl UpstreamClient for Socks5Client {
    async fn connect(
        &self,
        target: &SocketAddr,
        server: &SocketAddr,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Result<TcpStream> {
        // Connect properly marking the socket (PROXY_FWMARK) to bypass the TPROXY interceptor loop !
        let socket = create_marked_tcp_socket(server)?;
        let mut stream = socket.connect(*server).await?;

        // 1. Initial Handshake [VER, NMETHODS, METHODS...]
        let has_auth = username.is_some() && password.is_some();
        let methods = if has_auth {
            vec![0x00, 0x02] // NO AUTH and USERNAME/PASSWORD
        } else {
            vec![0x00] // NO AUTH ONLY
        };

        let mut req = vec![0x05, methods.len() as u8];
        req.extend_from_slice(&methods);
        stream.write_all(&req).await?;

        // Wait for server response [VER, METHOD]
        let mut resp = [0u8; 2];
        stream.read_exact(&mut resp).await?;

        if resp[0] != 0x05 {
            bail!("Invalid SOCKS version from server");
        }

        match resp[1] {
            0x00 => { /* No auth required */ }
            0x02 => {
                if !has_auth {
                    bail!("Server requires auth but no credentials provided");
                }
                Self::authenticate(&mut stream, username.unwrap(), password.unwrap()).await?;
            }
            0xFF => bail!("No acceptable auth methods supported by server"),
            _ => bail!("Unknown auth method requested: {}", resp[1]),
        }

        // 2. Send Connection Request
        let mut conn_req = vec![
            0x05, // VER
            0x01, // CMD (CONNECT)
            0x00, // RSV
        ];

        match target {
            SocketAddr::V4(addr) => {
                conn_req.push(0x01); // ATYP (IPv4)
                conn_req.extend_from_slice(&addr.ip().octets());
                conn_req.extend_from_slice(&addr.port().to_be_bytes());
            }
            SocketAddr::V6(addr) => {
                conn_req.push(0x04); // ATYP (IPv6)
                conn_req.extend_from_slice(&addr.ip().octets());
                conn_req.extend_from_slice(&addr.port().to_be_bytes());
            }
        }

        stream.write_all(&conn_req).await?;

        // 3. Read Server Reply
        let mut reply = [0u8; 4];
        stream.read_exact(&mut reply).await?;

        if reply[0] != 0x05 {
            bail!("Invalid SOCKS reply version");
        }

        if reply[1] != 0x00 {
            bail!("SOCKS5 Server connection failed with code: 0x{:02x}", reply[1]);
        }

        // Read the rest of the bound address (we don't strictly need it but must consume buffer)
        match reply[3] {
            0x01 => { // IPv4
                let mut buf = [0u8; 6]; // 4 ip + 2 port
                stream.read_exact(&mut buf).await?;
            }
            0x03 => { // DOMAINNAME
                let mut len = [0u8; 1];
                stream.read_exact(&mut len).await?;
                let mut buf = vec![0u8; len[0] as usize + 2]; // name + 2 port
                stream.read_exact(&mut buf).await?;
            }
            0x04 => { // IPv6
                let mut buf = [0u8; 18]; // 16 ip + 2 port
                stream.read_exact(&mut buf).await?;
            }
            _ => bail!("Unknown address type in SOCKS5 reply"),
        }

        Ok(stream)
    }
}

impl Socks5Client {
    async fn authenticate(stream: &mut TcpStream, user: &str, pass: &str) -> Result<()> {
        let u_bytes = user.as_bytes();
        let p_bytes = pass.as_bytes();

        if u_bytes.len() > 255 || p_bytes.len() > 255 {
            bail!("Username or password too long (max 255 chars)");
        }

        let mut req = vec![
            0x01, // Auth Version
            u_bytes.len() as u8,
        ];
        req.extend_from_slice(u_bytes);
        req.push(p_bytes.len() as u8);
        req.extend_from_slice(p_bytes);

        stream.write_all(&req).await?;

        let mut resp = [0u8; 2];
        stream.read_exact(&mut resp).await?;

        if resp[0] != 0x01 {
            bail!("Unknown auth version returned: {}", resp[0]);
        }

        if resp[1] != 0x00 {
            bail!("SOCKS5 Authentication failed (bad credentials)");
        }

        Ok(())
    }
}
