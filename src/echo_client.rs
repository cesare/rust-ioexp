use async_std::io::{self};
use async_std::net::TcpStream;
use async_std::prelude::*;

#[async_std::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(b"hello world").await?;

    let mut buf = [0u8; 1024];
    let _ = stream.read(&mut buf).await?;
    io::stdout().write(&buf).await?;

    Ok(())
}
