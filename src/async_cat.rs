use std::env::args;
use async_std::fs::File;
use async_std::io::{self, BufReader};
use async_std::prelude::*;
use std::vec::Vec;

async fn show(path: &str) -> Result<(), io::Error> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next().await {
        println!("{}", line?);
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
