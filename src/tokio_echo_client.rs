use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
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

#[tokio::main]
async fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let stream = TcpStream::connect(opt.bind_address()).await?;

    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(message) = lines.next_line().await? {
        stream.writable().await?;
        stream.try_write(message.as_ref())?;

        let mut buf = vec![0; 1024];
        loop {
            stream.readable().await?;
            match stream.try_read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    buf.truncate(n);
                    println!("reply = {:?}", buf);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => return Err(e.into())
            }
        }
    }

    Ok(())
}
