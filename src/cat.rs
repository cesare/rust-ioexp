use std::env::args;
use std::fs::File;
use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::vec::Vec;

struct FileInputStream<T: Read> {
    reader: T,
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
            Ok(0)  => None,
            Ok(n)  => Some(Ok(buf[..n].to_vec())),
        }
    }
}

impl From<File> for FileInputStream<File> {
    fn from(file: File) -> Self {
        FileInputStream { reader: file }
    }
}

impl From<io::Stdin> for FileInputStream<io::Stdin> {
    fn from(stdin: io::Stdin) -> Self {
        FileInputStream { reader: stdin }
    }
}

fn main() -> Result<(), io::Error> {
    let paths: Vec<String> = args().skip(1).collect();
    if paths.is_empty() {
        return FileInputStream::from(io::stdin()).show()
    }

    for path in paths {
        let file = File::open(path)?;
        FileInputStream::from(file).show()?;
    }

    Ok(())
}
