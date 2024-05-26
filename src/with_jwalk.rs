use crate::{get_path, Arguments, FileInfo, Key, MyResult};
use jwalk::{DirEntry, Parallelism, WalkDirGeneric};
use rayon::prelude::*;
use std::path::PathBuf;

/// Get all files into one vector.
///
/// Use jwalk.
pub fn get_all_files(arguments: &Arguments) -> MyResult<Vec<FileInfo>> {
    let path: PathBuf = get_path(arguments)?;

    let min_size: u64 = arguments.min_size;
    let max_size: u64 = arguments.max_size;

    let jwalk = WalkDirGeneric::<((), Option<FileInfo>)>::new(path)
        .min_depth(arguments.min_depth)
        .max_depth(arguments.max_depth)
        .parallelism(Parallelism::RayonNewPool(rayon::current_num_threads()))
        .skip_hidden(arguments.omit_hidden)
        .process_read_dir(move |_depth, _path, _read_dir_state, dir_entry_results| {
            analyze_dir_entry_results(dir_entry_results, min_size, max_size);
        });

    let all_files: MyResult<Vec<FileInfo>> = jwalk
        .into_iter()
        .flatten() // Result<DirEntry, Error> to DirEntry
        .filter_map(|dir_entry| dir_entry.client_state.map(Ok))
        .collect();

    all_files
}

type JwalkResults = Vec<Result<DirEntry<((), Option<FileInfo>)>, jwalk::Error>>;

// https://docs.rs/jwalk
// https://github.com/Byron/jwalk/blob/main/examples/du.rs
fn analyze_dir_entry_results(dir_entry_results: &mut JwalkResults, min_size: u64, max_size: u64) {
    // inode: “index nodes”
    // https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.ino

    /*
    // 1. Custom sort
    dir_entry_results
    .sort_by(|a, b| match (a, b) {
        (Ok(a), Ok(b)) => a.metadata().map(|m| m.ino()).unwrap_or(0).cmp(&b.metadata().map(|m| m.ino()).unwrap_or(0)),
        (Ok(_), Err(_)) => Ordering::Less,
        (Err(_), Ok(_)) => Ordering::Greater,
        (Err(_), Err(_)) => Ordering::Equal,
    });
    */

    // 3. Custom skip
    dir_entry_results
        //.iter_mut()
        .par_iter_mut() // rayon parallel iterator
        .flatten() // Result<DirEntry, Error> to DirEntry
        .filter(|dir_entry| dir_entry.file_type().is_file())
        .for_each(|dir_entry| {
            if let Ok(metadata) = dir_entry.metadata() {
                let file_size: u64 = metadata.len();
                //let inode_number: u64 = metadata.ino();

                if file_size >= min_size && file_size <= max_size {
                    let key = Key::new(file_size, None);
                    let path = dir_entry.path();
                    dir_entry.client_state = Some(FileInfo { key, path });
                } else {
                    dir_entry.client_state = None;
                };
            }
        });
}
