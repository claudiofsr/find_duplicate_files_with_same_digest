use crate::{get_path, Arguments, FileInfo, Key, MyResult};
use jwalk::{DirEntry, Parallelism, WalkDirGeneric};
use std::path::PathBuf;

/// Get all files into one vector.
///
/// Use jwalk.
pub fn get_all_files(arguments: &Arguments) -> MyResult<Vec<FileInfo>> {
    let path: PathBuf = get_path(arguments)?;

    let min_size: u64 = arguments.min_size;
    let max_size: u64 = arguments.max_size;

    let jwalk = WalkDirGeneric::<((), Option<FileInfo>)>::new(path)
        .skip_hidden(arguments.omit_hidden)
        .min_depth(arguments.min_depth)
        .max_depth(arguments.max_depth)
        .parallelism(Parallelism::RayonNewPool(rayon::current_num_threads()))
        .process_read_dir(move |_depth, _path, _read_dir_state, dir_entry_results| {
            process_dir_entries(dir_entry_results, min_size, max_size);
        });

    let all_files: Vec<FileInfo> = jwalk
        .into_iter()
        .flatten() // Result<DirEntry, Error> to DirEntry
        .flat_map(|dir_entry| dir_entry.client_state)
        .collect();

    Ok(all_files)
}

type JwalkResults = Vec<Result<DirEntry<((), Option<FileInfo>)>, jwalk::Error>>;

// https://docs.rs/jwalk
// https://github.com/Byron/jwalk/blob/main/examples/du.rs
fn process_dir_entries(dir_entry_results: &mut JwalkResults, min_size: u64, max_size: u64) {
    // inode: “index nodes”
    // https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.ino

    // cargo bench

    // 3. Custom skip
    dir_entry_results
        .iter_mut()
        .flatten() // Result<DirEntry, Error> to DirEntry
        .filter_map(|dir_entry| 
            if dir_entry.file_type().is_file() {
                dir_entry
                    .metadata()
                    .ok()
                    .map(|metadata| (dir_entry, metadata.len()))
            } else {
                None
            }
        )
        .filter(|(_dir_entry, file_size)| *file_size >= min_size && *file_size <= max_size)
        .for_each(|(dir_entry, file_size)| {
            let key = Key::new(file_size, None);
            let path = dir_entry.path();
            dir_entry.client_state = Some(FileInfo { key, path });
        });
}
