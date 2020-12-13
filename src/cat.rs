use std::env::args;
use std::fs::File;
use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::vec::Vec;

struct FileInputStream<T: Read> {
    reader: T,
}

impl FileInputStream<File> {
    pub fn new(file: File) -> FileInputStream<File> {
        FileInputStream { reader: file }
    }
}

impl FileInputStream<io::Stdin> {
    pub fn from_stdin() -> FileInputStream<io::Stdin> {
        FileInputStream { reader: io::stdin() }
    }
}

impl<T: Read> FileInputStream<T> {
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

impl<T: Read> Iterator for FileInputStream<T> {
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

fn main() -> Result<(), io::Error> {
    let paths: Vec<String> = args().skip(1).collect();
    if paths.is_empty() {
        return FileInputStream::from_stdin().show()
    }

    for path in paths {
        let file = File::open(path)?;
        FileInputStream::new(file).show()?;
    }

    Ok(())
}
