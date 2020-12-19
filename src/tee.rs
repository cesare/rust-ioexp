use std::env::args;
use async_std::fs::File;
use async_std::io::{self};
use async_std::prelude::*;
use futures::try_join;

async fn wait_for_input() -> Result<Option<String>, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).await
        .and_then(|n| if n == 0 { Ok(None) } else { Ok(Some(buffer)) })
}

#[async_std::main]
async fn main() -> Result<(), io::Error> {
    let arguments: Vec<String> = args().skip(1).collect();
    if arguments.is_empty() {
        println!("Usage: tee path");
        std::process::exit(111);
    }

    let path = &arguments[0];
    let mut file = File::create(path).await?;

    let mut stdout = io::stdout();

    while let Some(text) = wait_for_input().await? {
        let w1 = file.write_all(text.as_bytes());
        let w2 = stdout.write_all(text.as_bytes());
        let _ = try_join!(w1, w2);
    }

    Ok(())
}
