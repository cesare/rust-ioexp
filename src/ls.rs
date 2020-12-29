use std::env::args;
use std::fs::{self, DirEntry, Metadata};
use std::io::{self};
use std::os::unix::prelude::*;

struct Entry {
    filename: String,
    metadata: Metadata,
}

impl Entry {
    fn new(filename: &str, metadata: Metadata) -> Self {
        Entry {
            filename: filename.to_string(),
            metadata: metadata,
        }
    }

    fn from_direntry(de: DirEntry) -> Result<Self, io::Error> {
        let metadata = de.metadata()?;
        let filename = de.file_name();
        Ok(Self::new(&filename.to_string_lossy(), metadata))
    }

    fn description(&self) -> String {
        let mode = self.metadata.permissions().mode();
        format!("{:>016b} {:>6} {}", mode, self.metadata.len(), self.filename)
    }

    fn show(&self) {
        println!("{}", self.description());
    }
}

fn show_file(path: &str, metadata: Metadata) -> Result<(), io::Error> {
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    println!("{:>016b} {:>6} {}", mode, metadata.len(), path);
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
    for entry in collect_entries(path)? {
        Entry::from_direntry(entry)?.show();
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
