use async_std::io::{self};
use async_std::net::TcpStream;
use async_std::prelude::*;

async fn read_line() -> io::Result<Option<String>> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf).await? {
        0 => Ok(None),
        _ => Ok(Some(buf)),
    }
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    while let Some(message) = read_line().await? {
        stream.write_all(message.as_bytes()).await?;

        let mut buf = vec![0u8; 1024];
        let _ = stream.read(&mut buf).await?;
        io::stdout().write(&buf).await?;
    }

    Ok(())
}
