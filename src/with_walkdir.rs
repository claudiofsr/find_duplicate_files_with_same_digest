use crate::{get_path, Arguments, FileInfo, Key, MyResult};
use rayon::prelude::*;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

/// Get all files into one vector.
///
/// Use walkdir.
pub fn get_all_files(arguments: &Arguments) -> MyResult<Vec<FileInfo>> {
    let entries: Vec<DirEntry> = get_entries(arguments)?;

    let all_files: Vec<FileInfo> = entries
        .into_par_iter() // rayon parallel iterator
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let file_size: u64 = metadata.len();
            //let inode_number: u64 = metadata.ino();

            if arguments.size_is_included(file_size) {
                let key = Key::new(file_size, None);
                let path = entry.into_path();
                Some(FileInfo { key, path })
            } else {
                None
            }
        })
        .collect();

    Ok(all_files)
}

/// Get result: Vec<DirEntry>.
fn get_entries(arguments: &Arguments) -> MyResult<Vec<DirEntry>> {
    let dir_path: PathBuf = get_path(arguments)?;

    let entries: Vec<DirEntry> = WalkDir::new(dir_path)
        .min_depth(arguments.min_depth)
        .max_depth(arguments.max_depth)
        .into_iter()
        .filter_entry(|e| !arguments.omit_hidden || !is_hidden(e))
        .flatten() // Result<DirEntry, Error> to DirEntry
        .filter(|entry| entry.file_type().is_file())
        .collect();

    Ok(entries)
}

// https://github.com/BurntSushi/walkdir
// https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
/// Identify hidden files efficiently on unix.
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| entry.depth() != 0 && s.starts_with('.'))
}
