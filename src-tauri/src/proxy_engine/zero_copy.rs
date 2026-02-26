use anyhow::Result;
use std::os::unix::io::AsRawFd;
use tokio::net::TcpStream;

/// Perfoms a zero-copy relay between two TCP streams in both directions concurrently.
pub async fn bidirectional_splice(client: &mut TcpStream, server: &mut TcpStream) -> Result<(u64, u64)> {
    let (tx_bytes, rx_bytes) = tokio::try_join!(
        splice_copy(client, server),
        splice_copy(server, client)
    )?;
    Ok((tx_bytes, rx_bytes))
}

/// Zero-copy copies data from `read_sock` to `write_sock` using Linux `splice()` and a pipe.
/// Instead of allocating buffers in userspace RAM, it asks the kernel to move data directly.
async fn splice_copy(read_sock: &TcpStream, write_sock: &TcpStream) -> Result<u64> {
    let mut pipe_fds = [0 as libc::c_int; 2];
    if unsafe { libc::pipe2(pipe_fds.as_mut_ptr(), libc::O_NONBLOCK | libc::O_CLOEXEC) } < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    
    let pipe_read = pipe_fds[0];
    let pipe_write = pipe_fds[1];
    
    struct PipeGuard(libc::c_int, libc::c_int);
    impl Drop for PipeGuard {
        fn drop(&mut self) {
            unsafe {
                libc::close(self.0);
                libc::close(self.1);
            }
        }
    }
    let _guard = PipeGuard(pipe_read, pipe_write);
    
    let mut total_bytes = 0u64;
    
    loop {
        read_sock.readable().await?;
        
        // 1. Splice from source socket into pipe
        let bytes_in_pipe = match unsafe {
            libc::splice(
                read_sock.as_raw_fd(), std::ptr::null_mut(),
                pipe_write, std::ptr::null_mut(),
                65536,
                libc::SPLICE_F_NONBLOCK | libc::SPLICE_F_MOVE
            )
        } {
            n if n > 0 => n as usize,
            0 => break, // EOF
            -1 => {
                let err = std::io::Error::last_os_error();
                if err.kind() == std::io::ErrorKind::WouldBlock { continue; }
                return Err(err.into());
            }
            _ => unreachable!(),
        };

        // 2. Splice from pipe into destination socket
        let mut written = 0;
        while written < bytes_in_pipe {
            write_sock.writable().await?;
            match unsafe {
                libc::splice(
                    pipe_read, std::ptr::null_mut(),
                    write_sock.as_raw_fd(), std::ptr::null_mut(),
                    bytes_in_pipe - written,
                    libc::SPLICE_F_NONBLOCK | libc::SPLICE_F_MOVE
                )
            } {
                n if n > 0 => {
                    written += n as usize;
                    total_bytes += n as u64;
                },
                0 => anyhow::bail!("Write socket closed prematurely during splice"),
                -1 => {
                    let err = std::io::Error::last_os_error();
                    if err.kind() == std::io::ErrorKind::WouldBlock { continue; }
                    return Err(err.into());
                }
                _ => unreachable!(),
            }
        }
    }
    
    Ok(total_bytes)
}
