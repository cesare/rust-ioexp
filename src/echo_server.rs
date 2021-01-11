use async_std::io::{self};
use async_std::net::TcpListener;
use async_std::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "echo_server")]
struct Opt {
    #[structopt(short, long, default_value = "127.0.0.1")]
    bind: String,

    #[structopt(short, long, default_value = "8080")]
    port: u32,
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let bind_address = format!("{}:{}", opt.bind, opt.port);
    println!("Waiting for requests on {}", bind_address);

    let listener = TcpListener::bind(bind_address).await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let (reader, writer) = &mut (&stream, &stream);
        io::copy(reader, writer).await?;
    }

    Ok(())
}
