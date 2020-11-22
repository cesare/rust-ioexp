use std::io::{self};

fn wait_for_input() -> Option<String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(0) => None,
        Ok(_) => {
            Some(buffer)
        }
        Err(_) => None
    }
}

fn main() {
    while let Some(text) = wait_for_input() {
        print!("{}", text)
    }
}
