use anyhow::Result;
use async_std::net::{IpAddr, SocketAddr, TcpStream};
use async_std::sync::{Arc, Mutex};

/// Open a `TcpStream`.
pub async fn open_stream(ip: IpAddr, port: u16) -> Result<Arc<Mutex<TcpStream>>> {
    let stream = TcpStream::connect(SocketAddr::new(ip, port)).await?;

    Ok(Arc::new(Mutex::new(stream)))
}