use std::env::args;
use async_std::fs::{self, Metadata};
use async_std::io::{self};
use async_std::prelude::*;

async fn show_file(path: &str, _metadata: Metadata) -> Result<(), io::Error> {
    io::stdout().write_fmt(format_args!("{}\n", path)).await
}

async fn show_directory(path: &str) -> Result<(), io::Error> {
    let mut entries = fs::read_dir(path).await?;
    while let Some(result) = entries.next().await {
        let entry = result?;
        let metadata = entry.metadata().await?;
        show_file(&entry.file_name().to_string_lossy(), metadata).await?
    }
    Ok(())
}

async fn show(path: &str) -> Result<(), io::Error> {
    let metadata = fs::metadata(path).await?;
    if metadata.is_dir() {
        show_directory(path).await?
    } else {
        show_file(path, metadata).await?
    }

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), io::Error> {
    let arguments: Vec<String> = args().skip(1).collect();
    for path in arguments {
        show(&path).await?;
    }

    Ok(())
}
