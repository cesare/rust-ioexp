use std::env::args;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::iter::Iterator;
use std::path::Path;
use std::vec::Vec;

struct FileInputStream {
    reader: BufReader<File>,
}

impl FileInputStream {
    pub fn new(file: File) -> FileInputStream {
        let reader = BufReader::new(file);
        FileInputStream { reader: reader }
    }
}

impl Iterator for FileInputStream {
    type Item = Result<Vec<u8>, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 1024];
        match self.reader.read(&mut buf) {
            Err(e) => Some(Err(e)),
            Ok(n) => {
                if n == 0 { None } else { Some(Ok(buf[..n].to_vec())) }
            }
        }
    }
}

fn show(path_name: &str) -> Result<(), io::Error> {
    let path = Path::new(path_name);
    let file = File::open(path)?;
    let fis = FileInputStream::new(file);

    for result in fis {
        match result {
            Err(e) => { return Err(e) },
            Ok(content) => { io::stdout().write(&content)?; }
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
