use std::env::args;
use std::fs::{File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn show(path_name: &str) -> Result<(), io::Error> {
    let path = Path::new(path_name);
    let file = File::open(path)?;
    let f = BufReader::new(file);

    for result in f.lines() {
        let line = result?;
        println!("{}", line);
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    for name in args().skip(1) {
        show(&name)?
    }

    Ok(())
}
