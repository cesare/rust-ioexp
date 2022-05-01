use std::env::args;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let arguments: Vec<String> = args().skip(1).collect();
  if arguments.is_empty() {
    println!("Usage: httpget uri");
    std::process::exit(111);
  }

  let uri = &arguments[0];
  let mut response = reqwest::get(uri).await?;

  for (key, value) in response.headers() {
    io::stdout().write_all(format!("{}: {}\n", key, value.to_str()?).as_bytes()).await?;
  }

  while let Some(chunk) = response.chunk().await? {
    io::stdout().write_all(chunk.as_ref()).await?;
  }

  Ok(())
}
