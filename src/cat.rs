use std::env::args;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

fn show(path_name: &str) -> Result<(), io::Error> {
    let path = Path::new(path_name);
    let file = File::open(path)?;
    let mut f = BufReader::new(file);

    let mut buf = [0; 1024];
    loop {
        match f.read(&mut buf)? {
            0 => break,
            n => {
                let content = &buf[..n];
                io::stdout().write(content)?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    for name in args().skip(1) {
        show(&name)?
    }

    Ok(())
}
