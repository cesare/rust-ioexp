use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::net::tcp::OwnedReadHalf;
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

struct InputStream<R> {
    reader: BufReader<R>,
}

impl<R: AsyncRead + Unpin> InputStream<R> {
    fn new(read: R) -> Self {
        InputStream {
            reader: BufReader::new(read),
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

async fn wait_for_responses(read: OwnedReadHalf) -> Result<(), io::Error> {
    let mut server_stream = InputStream::new(read);
    while let Some(message) = server_stream.read_line().await? {
        io::stdout().write(message.as_bytes()).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let tcpstream = TcpStream::connect(opt.bind_address()).await?;
    let (r, mut w) = tcpstream.into_split();

    let join_handle = tokio::spawn(async move {
        let _ = wait_for_responses(r).await;
    });

    let mut input_stream = InputStream::new(io::stdin());
    while let Some(message) = input_stream.read_line().await? {
        w.write_all(message.as_ref()).await?;
    }

    w.shutdown().await?;
    join_handle.await?;

    Ok(())
}
