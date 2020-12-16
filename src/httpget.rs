use std::env::args;
use hyper::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let arguments: Vec<String> = args().skip(1).collect();
  if arguments.is_empty() {
    println!("Usage: httpget uri");
    std::process::exit(111);
  }

  let client = Client::new();
  let uri = arguments[0].parse()?;
  let response = client.get(uri).await?;

  println!("Response: {}", response.status());
  Ok(())
}
