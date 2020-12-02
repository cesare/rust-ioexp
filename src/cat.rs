use std::env::args;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::iter::Iterator;
use std::path::Path;
use std::vec::Vec;

struct FileInputStream {
    reader: BufReader<Box<dyn Read>>,
}

impl FileInputStream {
    pub fn new(file: Box<dyn Read>) -> FileInputStream {
        let reader = BufReader::new(file);
        FileInputStream { reader: reader }
    }

    pub fn from_stdin() -> FileInputStream {
        let read = Box::new(Box::new(std::io::stdin()));
        Self::new(read)
    }

    pub fn show(self) -> Result<(), io::Error> {
        for result in self {
            match result {
                Err(e) => { return Err(e) },
                Ok(content) => { io::stdout().write(&content)?; }
            }
        }
        Ok(())
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
    let fis = FileInputStream::new(Box::new(file));
    fis.show()
}

fn main() -> Result<(), io::Error> {
    if args().count() == 1 {
        let fis = FileInputStream::from_stdin();
        return fis.show()
    }

    for name in args().skip(1) {
        show(&name)?
    }

    Ok(())
}
