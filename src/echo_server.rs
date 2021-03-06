use async_std::io::{self};
use async_std::net::{TcpListener, TcpStream};
use futures::stream::StreamExt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "echo_server")]
struct Opt {
    #[structopt(short, long, default_value = "127.0.0.1")]
    bind: String,

    #[structopt(short, long, default_value = "8080")]
    port: u32,
}

impl Opt {
    fn bind_address(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }
}

async fn handle(stream: TcpStream) {
    let (reader, writer) = &mut (&stream, &stream);
    let _ = io::copy(reader, writer).await;
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let bind_address = opt.bind_address();
    println!("Waiting for requests on {}", bind_address);

    let listener = TcpListener::bind(bind_address).await?;
    listener.incoming().for_each_concurrent(None, |tcpstream| async move {
        let tcpstream = tcpstream.unwrap();
        handle(tcpstream).await;
    }).await;

    Ok(())
}
