use std::env::args;
use std::fmt;
use std::fs::{self, DirEntry, Metadata};
use std::io::{self};
use std::os::unix::prelude::*;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use users::{get_user_by_uid};

#[derive(PartialEq)]
enum PermissionTarget {
    Owner,
    Group,
    Other,
}

struct Filesize(u64);

impl fmt::Display for Filesize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let units = ["", "k", "M", "G", "T", "P", "E"];
        let ns = (0..6).map(|i| 1 << (i * 10))
            .map(|d| self.0 as f64 / d as f64)
            .map(|f| f.round() as u64);
        let (u, n) = units.iter().zip(ns).find(|(_u, n)| n < &1024).unwrap();
        write!(f, "{:>4}{:>1}", n, u)
    }
}

struct Permissions {
    mode: u32,
}

impl Permissions {
    fn new(mode: u32) -> Self {
        Permissions {
            mode: mode,
        }
    }

    fn file_type(&self) -> char {
        let mask = 0o170000;
        let patterns: Vec<(u32, char)> = vec![
            (0o010000, 'p'), // named pipe
            (0o020000, 'c'), // character special
            (0o040000, 'd'), // directory
            (0o060000, 'b'), // block special
            (0o120000, 'l'), // symbolic link
            (0o140000, 's'), // socket
        ];

        match patterns.iter().find(|(b, _c)| self.mode & mask == *b).map(|(_b, c)| *c) {
            Some(c) => c,
            None => '.'
        }
    }

    fn execution_mode(&self, mode_for_target: u32, target: PermissionTarget) -> u8 {
        let cs = [
            ['-', 'x'],
            ['S', 's'],
            ['T', 't'],
        ];

        let executable = mode_for_target & 0b001 == 0b001;
        let sticky = mode_for_target & 0o1000 == 0o1000;

        let x = if executable { 1 } else { 0 };
        let y = match target {
            PermissionTarget::Owner => {
                if self.is_setuid() { 1 } else { 0 }
            }
            PermissionTarget::Group => {
                if self.is_setgid() { 1 } else { 0 }
            }
            PermissionTarget::Other => {
                if sticky { 2 } else { 0 }
            }
        };
        cs[y][x] as u8
    }

    fn mode_expression(&self, target: PermissionTarget) -> String {
        let mode = self.mode;
        let mode_for_target =
            match target {
                PermissionTarget::Owner => mode >> 6,
                PermissionTarget::Group => mode >> 3,
                PermissionTarget::Other => mode,
            };

        let mut cs: Vec<u8> = vec!['-' as u8; 3];
        if mode_for_target & 0b100 == 0b100 {
            cs[0] = 'r' as u8
        }
        if mode_for_target & 0b010 == 0b010 {
            cs[1] = 'w' as u8
        }
        cs[2] = self.execution_mode(mode_for_target, target);
        String::from_utf8_lossy(&cs).to_string()
    }

    fn is_setuid(&self) -> bool {
        self.mode & 0o4000 == 0o4000
    }

    fn is_setgid(&self) -> bool {
        self.mode & 0o2000 == 0o2000
    }
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode_for_owner = self.mode_expression(PermissionTarget::Owner);
        let mode_for_group = self.mode_expression(PermissionTarget::Group);
        let mode_for_other = self.mode_expression(PermissionTarget::Other);
        let filetype = self.file_type();
        write!(f, "{}{}{}{}", filetype, mode_for_owner, mode_for_group, mode_for_other)
    }
}

struct Timestamp(SystemTime);

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ts = DateTime::<Local>::from(self.0).format("%Y-%m-%d %H:%M");
        write!(f, "{}", ts)
    }
}

struct User(u32);

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let uid = self.0;
        match get_user_by_uid(uid) {
            Some(user) => write!(f, "{}", user.name().to_string_lossy()),
            None => write!(f, "{:03}", uid),
        }
    }
}

struct Entry {
    filename: String,
    metadata: Metadata,
    size: Filesize,
    permissions: Permissions,
}

impl Entry {
    fn new(filename: &str, metadata: &Metadata) -> Self {
        Entry {
            filename: filename.to_string(),
            metadata: metadata.to_owned(),
            size: Filesize(metadata.len()),
            permissions: Permissions::new(metadata.permissions().mode()),
        }
    }

    fn from_direntry(de: DirEntry) -> Result<Self, io::Error> {
        let metadata = de.metadata()?;
        let filename = de.file_name();
        Ok(Self::new(&filename.to_string_lossy(), &metadata))
    }

    fn show(&self) -> Result<(), io::Error> {
        let modified_at = Timestamp(self.metadata.modified()?).to_string();
        let user = User(self.metadata.uid());
        println!("{} {} {} {} {}", self.permissions, self.size, user, modified_at, self.filename);

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
    let metadata = fs::symlink_metadata(path)?;
    if metadata.is_dir() {
        show_directory(path)
    } else {
        Entry::new(path, &metadata).show()?;
        Ok(())
    }
}

fn main() -> Result<(), io::Error> {
    let mut arguments: Vec<String> = args().skip(1).collect();
    if arguments.is_empty() {
        arguments.push(".".to_string())
    }

    for path in arguments {
        show(&path)?;
    }

    Ok(())
}
