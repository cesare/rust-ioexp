use std::env::args;
use std::pin::Pin;
use async_std::fs::File;
use async_std::io::{self, Read};
use async_std::prelude::*;
use async_std::task::{Context, Poll};
use std::vec::Vec;
use pin_project::pin_project;

#[pin_project]
struct FileInputStream<T: Read> {
    file: T,
}

impl FileInputStream<File> {
    fn new(file: File) -> FileInputStream<File> {
        FileInputStream { file: file }
    }
}

impl FileInputStream<File> {
    async fn show(&mut self) -> Result<(), io::Error> {
        while let Some(content) = self.next().await {
            io::stdout().write(&content?).await?;
        }
        Ok(())
    }
}

impl Stream for FileInputStream<File> {
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
    FileInputStream::new(file).show().await
}

#[async_std::main]
async fn main() -> Result<(), io::Error> {
    let paths: Vec<String> = args().skip(1).collect();

    for path in paths {
        show(&path).await?
    }

    Ok(())
}
