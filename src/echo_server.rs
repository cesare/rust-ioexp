use async_std::io::{self};
use async_std::net::TcpListener;
use async_std::prelude::*;

#[async_std::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let (reader, writer) = &mut (&stream, &stream);
        io::copy(reader, writer).await?;
    }

    Ok(())
}
