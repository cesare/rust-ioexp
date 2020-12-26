use std::env::args;
use std::fs::{self, Metadata};
use std::io::{self};

fn show_file(path: &str, _metadata: Metadata) -> Result<(), io::Error> {
    println!("{}", path);
    Ok(())
}

fn show_directory(path: &str) -> Result<(), io::Error> {
    let mut entries = fs::read_dir(path)?;
    while let Some(result) = entries.next() {
        let entry = result?;
        let metadata = entry.metadata()?;
        show_file(&entry.file_name().to_string_lossy(), metadata)?
    }
    Ok(())
}

fn show(path: &str) -> Result<(), io::Error> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        show_directory(path)?
    } else {
        show_file(path, metadata)?
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let arguments: Vec<String> = args().skip(1).collect();
    for path in arguments {
        show(&path)?;
    }

    Ok(())
}
