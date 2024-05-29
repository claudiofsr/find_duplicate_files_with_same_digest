use crate::{
    add_thousands_separator,
    args::{Arguments, ResultFormat::*},
    split_and_insert,
    structures::group_info::GroupExtension,
    Algorithm, GroupInfo, MyResult, SEPARATOR,
};
use serde::Serialize;
use std::thread;

/// Summarize information for all files found in the directory
#[derive(Debug, Default, Clone, Serialize)]
pub struct TotalInfo {
    /// Hashing algorithm
    #[serde(rename = "Hashing algorithm")]
    pub algorithm: Algorithm,
    /// Total number of files found in the directory
    #[serde(rename = "Total number of files")]
    pub total_num_files: usize,
    /// Total number of identical files
    #[serde(rename = "Total number of identical files")]
    pub total_num_identical: usize,
    /// Total number of different blake3 hashes
    #[serde(rename = "Total number of different hashes")]
    pub total_num_hashes: usize,
    /// Total size of identical files
    #[serde(
        rename = "Total size of identical files",
        serialize_with = "add_thousands_separator"
    )]
    pub total_size: usize,
}

impl TotalInfo {
    /// Get the summary information.
    pub fn get_summary(
        identical_hash: &[GroupInfo],
        arguments: &Arguments,
        total_num_files: usize,
    ) -> Self {
        let (result_display, result_total_info) = thread::scope(|s| {
            let thread_a =
                s.spawn(|| -> MyResult<()> { identical_hash.print_identical_files(arguments) });
            let thread_b = s.spawn(|| -> TotalInfo {
                identical_hash.get_total_info(arguments, total_num_files)
            });

            // Wait for background thread to complete.
            // Call join() on each handle to make sure all the threads finish.
            // join() returns immediately when the associated thread completes.
            (thread_a.join(), thread_b.join())
        });

        let (_display, total_info) = match (result_display, result_total_info) {
            (Ok(display), Ok(total_info)) => (display, total_info),
            _ => panic!("thread::scope failed!"),
        };

        total_info
    }

    /// Print the identicald files information.
    pub fn print_summary(&self, arguments: &Arguments) -> MyResult<()> {
        match &arguments.result_format {
            Json => {
                // Serialize TotalInfo to a JSON string.
                let serialized = serde_json::to_string_pretty(&self)?;
                println!("{serialized}\n");
            }
            Yaml => {
                // Serialize GroupInfo to a YAML string.
                let serialized = serde_yaml::to_string(&self)?;
                println!("{serialized}");
            }
            Personal => {
                println!("Hashing algorithm: {}", arguments.algorithm); // or self.algorithm
                println!("Total number of files: {}", self.total_num_files);
                println!(
                    "Total number of identical files: {}",
                    self.total_num_identical
                );
                println!(
                    "Total number of different hashes: {}",
                    self.total_num_hashes
                );
                println!(
                    "Total size of identical files: {} bytes\n",
                    split_and_insert(self.total_size, SEPARATOR)
                );
            }
        }
        Ok(())
    }
}
