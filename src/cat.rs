use std::env::args;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;
use std::vec::Vec;

fn blockread(reader: &mut dyn Read) -> Result<Option<Vec<u8>>, io::Error> {
    let mut buf = [0; 1024];
    match reader.read(&mut buf)? {
        0 => Ok(None),
        n => Ok(Some(buf[..n].to_vec()))
    }
}

fn show(path_name: &str) -> Result<(), io::Error> {
    let path = Path::new(path_name);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    loop {
        match blockread(&mut reader)? {
            None => break,
            Some(content) => {
                io::stdout().write(&content)?;
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
