use std::io::{self};

fn echo() -> Option<usize> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(0) => None,
        Ok(n) => {
            print!("{}", buffer);
            Some(n)
        }
        Err(_) => None
    }
}

fn main() {
    loop {
        match echo() {
            Some(_) => {}
            None => { break; }
        }
    }
}
