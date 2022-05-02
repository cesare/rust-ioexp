use async_std::io::{self};

async fn wait_for_input() -> Result<Option<String>, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).await
        .map(|n| if n == 0 { None } else { Some(buffer) })
}

#[async_std::main]
async fn main() -> Result<(), io::Error> {
    while let Some(text) = wait_for_input().await? {
        print!("{}", text)
    }

    Ok(())
}
