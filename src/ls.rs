use std::env::args;
use std::fs::{self, DirEntry, Metadata};
use std::io::{self};
use std::os::unix::prelude::*;
use chrono::{DateTime, Local};

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

    fn description(&self) -> Result<String, io::Error> {
        let mode = self.metadata.permissions().mode();
        let modified = DateTime::<Local>::from(self.metadata.modified()?).format("%Y-%m-%d %H:%M");
        Ok(format!("{:>016b} {:>6} {} {}", mode, self.metadata.len(), modified, self.filename))
    }

    fn show(&self) -> Result<(), io::Error> {
        println!("{}", self.description()?);
        Ok(())
    }
}

fn collect_entries(path: &str) -> Result<Vec<Entry>, io::Error> {
    let mut entries: Vec<Entry> = fs::read_dir(path)?
        .filter_map(|result| result.ok())
        .map(|de| Entry::from_direntry(de))
        .filter_map(|result| result.ok())
        .collect();
    entries.sort_by(|a, b| a.filename.cmp(&b.filename));
    Ok(entries)
}

fn show_directory(path: &str) -> Result<(), io::Error> {
    for entry in collect_entries(path)? {
        entry.show()?;
    }
    Ok(())
}

fn show(path: &str) -> Result<(), io::Error> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        show_directory(path)
    } else {
        Entry::new(path, metadata).show()?;
        Ok(())
    }
}

fn main() -> Result<(), io::Error> {
    let arguments: Vec<String> = args().skip(1).collect();
    for path in arguments {
        show(&path)?;
    }

    Ok(())
}
