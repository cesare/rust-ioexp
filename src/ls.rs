use std::env::args;
use std::fs::{self, DirEntry, Metadata};
use std::io::{self};

fn show_file(path: &str, _metadata: Metadata) -> Result<(), io::Error> {
    println!("{}", path);
    Ok(())
}

fn collect_entries(path: &str) -> Result<Vec<DirEntry>, io::Error> {
    let mut entries: Vec<DirEntry> = fs::read_dir(path)?
        .filter_map(|result| result.ok())
        .collect();
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(entries)
}

fn show_directory(path: &str) -> Result<(), io::Error> {
    let entries = collect_entries(path)?;
    for entry in entries {
        let metadata = entry.metadata()?;
        show_file(&entry.file_name().to_string_lossy(), metadata)?
    }
    Ok(())
}

fn show(path: &str) -> Result<(), io::Error> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        show_directory(path)
    } else {
        show_file(path, metadata)
    }
}

fn main() -> Result<(), io::Error> {
    let arguments: Vec<String> = args().skip(1).collect();
    for path in arguments {
        show(&path)?;
    }

    Ok(())
}
