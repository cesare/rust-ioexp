use std::env::args;
use std::fs::{self, DirEntry, Metadata};
use std::io::{self};
use std::os::unix::prelude::*;

fn show_entry(entry: &DirEntry) -> Result<(), io::Error> {
    let filename = entry.file_name();
    let mode = entry.metadata()?.permissions().mode();
    println!("{:>016b} {}", mode, filename.to_string_lossy());
    Ok(())
}

fn show_file(path: &str, metadata: Metadata) -> Result<(), io::Error> {
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    println!("{:>016b} {}", mode, path);
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
        show_entry(&entry)?;
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
