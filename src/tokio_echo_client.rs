use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
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

struct InputStream {
    reader: BufReader<io::Stdin>,
}

impl InputStream {
    fn new() -> Self {
        InputStream {
            reader: BufReader::new(io::stdin()),
        }
    }

    async fn read_line(&mut self) -> io::Result<Option<String>> {
        let mut buf = String::new();
        match self.reader.read_line(&mut buf).await? {
            0 => Ok(None),
            _ => Ok(Some(buf)),
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let tcpstream = TcpStream::connect(opt.bind_address()).await?;
    let (mut r, mut w) = tcpstream.into_split();

    let mut input_stream = InputStream::new();
    while let Some(message) = input_stream.read_line().await? {
        w.write_all(message.as_ref()).await?;

        let mut buf = vec![0u8; 1024];
        let _n = r.read(&mut buf).await?;
        io::stdout().write(&buf).await?;
    }

    Ok(())
}
