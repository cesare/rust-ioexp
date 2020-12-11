use std::env::args;
use std::pin::Pin;
use async_std::fs::File;
use async_std::io::{self};
use async_std::prelude::*;
use async_std::task::{Context, Poll};
use std::vec::Vec;
use pin_project::pin_project;

#[pin_project]
struct FileInputStream {
    file: File,
}

impl FileInputStream {
    fn new(file: File) -> FileInputStream {
        FileInputStream { file: file }
    }
}

impl Stream for FileInputStream {
    type Item = Result<Vec<u8>, io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = [0; 1024];
        let mut f = self.project().file.read(&mut buf);
        match Pin::new(&mut f).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(0))  => { Poll::Ready(None) },
            Poll::Ready(Ok(n))  => { Poll::Ready(Some(Ok(buf[..n].to_vec()))) },
            Poll::Ready(Err(e)) => { Poll::Ready(Some(Err(e)))},
        }
    }
}

async fn show(path: &str) -> Result<(), io::Error> {
    let file = File::open(path).await?;
    let mut stream = FileInputStream::new(file);

    while let Some(content) = stream.next().await {
        io::stdout().write(&content?).await?;
    }

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), io::Error> {
    let paths: Vec<String> = args().skip(1).collect();

    for path in paths {
        show(&path).await?
    }

    Ok(())
}
