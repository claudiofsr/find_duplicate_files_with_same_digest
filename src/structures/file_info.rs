use crate::{Arguments, GroupInfo, Key};
use hashbrown::HashMap;
use rayon::prelude::*;
use std::path::PathBuf;

/// Individual file information
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// Key Information
    pub key: Key,
    /// File Path
    pub path: PathBuf,
}

pub trait FileExtension {
    /// Get two or more files with same key: (size, `Option<hash>`)
    fn get_grouped_files(&self, arguments: &Arguments, procedure: u8) -> Vec<GroupInfo>;
}

impl FileExtension for [FileInfo] {
    fn get_grouped_files(&self, arguments: &Arguments, procedure: u8) -> Vec<GroupInfo> {
        // minimum and maximum frequency (number of identical files)
        let min_frequency: usize = arguments.min_frequency as usize;
        let max_frequency: usize = arguments.max_frequency as usize;

        let mut group_by: HashMap<Key, Vec<PathBuf>> = HashMap::new();

        self.iter().for_each(|file_info| {
            group_by
                // key: (size, Option<hash>), value: paths
                .entry(file_info.key.clone())
                // If there's no entry for the key, create a new Vec and return a mutable ref to it
                .or_default()
                // and insert the item onto the Vec
                .push(file_info.path.clone())
        });

        /*
        // Group By Parallel Mode with 'MapReduce'
        let group_by: HashMap<Key, Vec<PathBuf>> = self
            .par_iter() // rayon: parallel iterator
            .map(|file_info| (file_info.key.clone(), file_info.path.clone()))
            .fold(
                HashMap::new,
                |mut accumulator: HashMap<Key, Vec<PathBuf>>, (key, path)| {
                    accumulator
                        // key: (size, Option<hash>), value: paths
                        .entry(key)
                        // If there's no entry for the key, create a new Vec and return a mutable ref to it
                        .or_default()
                        // and insert the item onto the Vec
                        .push(path);

                    accumulator
                },
            )
            .reduce(HashMap::new, |mut hashmap_a, hashmap_b| {
                // Merge two HashMaps
                hashmap_b.into_iter().for_each(|(key_b, value_b)| {
                    hashmap_a.entry(key_b).or_default().extend(value_b);
                });

                hashmap_a
            });
        */

        // Converting group_by to vector
        let grouped_files: Vec<GroupInfo> = group_by
            .into_par_iter() // rayon parallel iterator
            .filter(|(_key, paths)| {
                // Filter identical files with same key
                // procedure 1: filter only by size
                // procedure 2: filter by size and by hash of the first bytes
                // procedure 3: filter by size and by hash of the entire file
                if procedure <= 2 {
                    paths.len() >= min_frequency
                } else {
                    paths.len() >= min_frequency && paths.len() <= max_frequency
                }
            })
            .map(|(key, paths)| {
                let num_file = paths.len();
                let sum_size = key.size * num_file;
                GroupInfo {
                    key,
                    paths,
                    num_file,
                    sum_size,
                }
            })
            .collect();

        grouped_files
    }
}
