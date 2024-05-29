use criterion::{criterion_group, criterion_main, Criterion};
use find_identical_files::{FileInfo, Key};
use jwalk::{DirEntry, Parallelism, WalkDirGeneric};

type JwalkResults = Vec<Result<DirEntry<((), Option<FileInfo>)>, jwalk::Error>>;

// inode: “index nodes”
// https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.ino

// https://docs.rs/jwalk
// https://github.com/Byron/jwalk/blob/main/examples/du.rs

pub fn process_dir_entries_v1(dir_entry_results: &mut JwalkResults, min_size: u64, max_size: u64) {
    // 3. Custom skip
    dir_entry_results
        .iter_mut()
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
                }
            }
        });
}

pub fn process_dir_entries_v2(dir_entry_results: &mut JwalkResults, min_size: u64, max_size: u64) {
    // 3. Custom skip
    dir_entry_results
        .iter_mut()
        .for_each(|dir_entry_result| {
            if let Ok(dir_entry) = dir_entry_result {
                if dir_entry.file_type.is_file() {
                    if let Ok(metadata) = dir_entry.metadata() {
                        let file_size: u64 = metadata.len();
                        //let inode_number: u64 = metadata.ino();
        
                        if file_size >= min_size && file_size <= max_size {
                            let key = Key::new(file_size, None);
                            let path = dir_entry.path();
                            dir_entry.client_state = Some(FileInfo { key, path });
                        }
                    }
                }
            }
        });
}

pub fn process_dir_entries_v3(dir_entry_results: &mut JwalkResults, min_size: u64, max_size: u64) {
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

// https://github.com/Byron/jwalk/blob/main/benches/walk_benchmark.rs

fn benchmark_process_dir_entries(c: &mut Criterion) {

    let path = "~";
    let min_size = 0;
    let max_size = 10_000_000;

    let mut group = c.benchmark_group("Jwalk Dir Entries");

    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(60));
    group.sample_size(10);

    group.bench_function("process_dir_entries v1", |b| {
        b.iter(|| {
            for _ in WalkDirGeneric::<((), Option<FileInfo>)>::new(path)
                .parallelism(Parallelism::RayonNewPool(rayon::current_num_threads()))
                .process_read_dir(move |_depth, _path, _read_dir_state, dir_entry_results| {
                    process_dir_entries_v1(dir_entry_results, min_size, max_size);
                })
            {}
        })
    });

    group.bench_function("process_dir_entries v2", |b| {
        b.iter(|| {
            for _ in WalkDirGeneric::<((), Option<FileInfo>)>::new(path)
                .parallelism(Parallelism::RayonNewPool(rayon::current_num_threads()))
                .process_read_dir(move |_depth, _path, _read_dir_state, dir_entry_results| {
                    process_dir_entries_v2(dir_entry_results, min_size, max_size);
                })
            {}
        })
    });

    group.bench_function("process_dir_entries v3", |b| {
        b.iter(|| {
            for _ in WalkDirGeneric::<((), Option<FileInfo>)>::new(path)
                .parallelism(Parallelism::RayonNewPool(rayon::current_num_threads()))
                .process_read_dir(move |_depth, _path, _read_dir_state, dir_entry_results| {
                    process_dir_entries_v3(dir_entry_results, min_size, max_size);
                })
            {}
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_process_dir_entries);
criterion_main!(benches);
