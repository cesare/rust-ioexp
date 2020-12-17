use std::env::args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let arguments: Vec<String> = args().skip(1).collect();
  if arguments.is_empty() {
    println!("Usage: httpget uri");
    std::process::exit(111);
  }

  let uri = &arguments[0];
  let response = reqwest::get(uri).await?;

  println!("Response: {}", response.status());
  Ok(())
}
