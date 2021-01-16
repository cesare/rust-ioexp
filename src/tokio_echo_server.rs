use tokio::io::{self};
use tokio::net::{TcpListener, TcpStream};
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
    let (mut reader, mut writer) = stream.into_split();
    let _ = io::copy(&mut reader, &mut writer).await;
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let bind_address = opt.bind_address();
    println!("Waiting for requests on {}", bind_address);

    let listener = TcpListener::bind(bind_address).await?;
    loop {
        let (tcpstream, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle(tcpstream).await;
        });
    }
}
