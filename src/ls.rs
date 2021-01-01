use std::env::args;
use std::fs::{self, DirEntry, Metadata};
use std::io::{self};
use std::os::unix::prelude::*;
use chrono::{DateTime, Local};
use users::{get_user_by_uid};

#[derive(PartialEq)]
enum PermissionTarget {
    Owner,
    Group,
    Other,
}

struct Entry {
    filename: String,
    metadata: Metadata,
    mode: u32,
}

impl Entry {
    fn new(filename: &str, metadata: &Metadata) -> Self {
        Entry {
            filename: filename.to_string(),
            metadata: metadata.to_owned(),
            mode: metadata.permissions().mode(),
        }
    }

    fn from_direntry(de: DirEntry) -> Result<Self, io::Error> {
        let metadata = de.metadata()?;
        let filename = de.file_name();
        Ok(Self::new(&filename.to_string_lossy(), &metadata))
    }

    fn username(&self) -> String {
        let uid = self.metadata.uid();
        match get_user_by_uid(uid) {
            Some(user) => user.name().to_string_lossy().to_string(),
            None => format!("{:03}", uid),
        }
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
        if mode_for_target & 0b001 == 0b001 {
            cs[2] =
                if target == PermissionTarget::Owner && self.is_setuid() {
                    's' as u8
                } else if target == PermissionTarget::Group && self.is_setgid() {
                    's' as u8
                } else {
                    'x' as u8
                }
        }
        String::from_utf8_lossy(&cs).to_string()
    }

    fn permission_mode(&self) -> String {
        let mode = self.mode;
        let mode_for_owner = self.mode_expression(PermissionTarget::Owner);
        let mode_for_group = self.mode_expression(PermissionTarget::Group);
        let mode_for_other = self.mode_expression(PermissionTarget::Other);
        let filetype = (mode >> 9) & 0xff;
        format!("{:08b} {}{}{}", filetype, mode_for_owner, mode_for_group, mode_for_other)
    }

    fn is_setuid(&self) -> bool {
        self.mode & 0o4000 == 0o4000
    }

    fn is_setgid(&self) -> bool {
        self.mode & 0o2000 == 0o2000
    }

    fn modified_at(&self) -> Result<String, io::Error> {
        let modified = DateTime::<Local>::from(self.metadata.modified()?).format("%Y-%m-%d %H:%M");
        Ok(modified.to_string())
    }

    fn filesize(&self) -> String {
        format!("{:>6}", self.metadata.len())
    }

    fn description(&self) -> Result<String, io::Error> {
        Ok(format!("{} {} {} {} {}", self.permission_mode(), self.filesize(), self.username(), self.modified_at()?, self.filename))
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
        Entry::new(path, &metadata).show()?;
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
