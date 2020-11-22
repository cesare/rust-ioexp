use std::io::{self};

fn wait_for_input() -> Result<Option<String>, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
        .and_then(|n| if n == 0 { Ok(None) } else { Ok(Some(buffer)) })
}

fn main() -> Result<(), io::Error> {
    while let Some(text) = wait_for_input()? {
        print!("{}", text)
    }

    Ok(())
}
