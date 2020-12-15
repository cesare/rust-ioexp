use std::env::args;
use std::pin::Pin;
use async_std::fs::File;
use async_std::io::{self, Read};
use async_std::prelude::*;
use async_std::task::{Context, Poll};
use pin_project::pin_project;

#[pin_project]
struct FileInputStream<T: Read + Unpin> {
    reader: T,
}

impl<T: Read + Unpin> FileInputStream<T> {
    async fn show(&mut self) -> Result<(), io::Error> {
        while let Some(content) = self.next().await {
            io::stdout().write(&content?).await?;
        }
        Ok(())
    }
}

impl<T: Read + Unpin> Stream for FileInputStream<T> {
    type Item = Result<Vec<u8>, io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = [0; 1024];
        let mut f = self.project().reader.read(&mut buf);
        match Pin::new(&mut f).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(0))  => { Poll::Ready(None) },
            Poll::Ready(Ok(n))  => { Poll::Ready(Some(Ok(buf[..n].to_vec()))) },
            Poll::Ready(Err(e)) => { Poll::Ready(Some(Err(e)))},
        }
    }
}

impl From<File> for FileInputStream<File> {
    fn from(file: File) -> Self {
        FileInputStream { reader: file }
    }
}

impl From<io::Stdin> for FileInputStream<io::Stdin> {
    fn from(stdin: io::Stdin) -> Self {
        FileInputStream { reader: stdin }
    }
}

#[async_std::main]
async fn main() -> Result<(), io::Error> {
    let paths: Vec<String> = args().skip(1).collect();
    if paths.is_empty() {
        return FileInputStream::from(io::stdin()).show().await
    }

    for path in paths {
        let file = File::open(path).await?;
        FileInputStream::from(file).show().await?
    }

    Ok(())
}
