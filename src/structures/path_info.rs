use rust_xlsxwriter::XlsxSerialize;
use serde::{
    Serialize,
    //Serializer,
};
use std::path::PathBuf;

use crate::excel::get_xlsx_format;

/// File Information including path
///
/// rust_xlsxwriter: Working with Serde
///
/// <https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/serializer/index.html>
#[derive(XlsxSerialize, Serialize)]
#[xlsx(
    //header_format = get_xlsx_format("header"),
    table_default
)]
pub struct PathInfo {
    /// File size (in bytes)
    #[serde(rename = "File size (bytes)")]
    #[xlsx(value_format = get_xlsx_format("integer"))]
    pub size: usize,

    /// Hash
    #[serde(rename = "Hash")] // serialize_with = "add_quotes"
    #[xlsx(value_format = get_xlsx_format("center"))]
    pub hash: Option<String>,

    /// File Paths
    #[serde(rename = "Path")]
    #[xlsx(value_format = get_xlsx_format("default"))]
    pub path: PathBuf,

    /// Frequency (number of identical files) with the same size and hash
    #[serde(rename = "Frequency")]
    #[xlsx(value_format = get_xlsx_format("integer"))]
    pub num_file: usize,

    /// Sum of individual file sizes declared in paths
    #[serde(rename = "Sum of file sizes (bytes)")]
    #[xlsx(value_format = get_xlsx_format("integer"))]
    pub sum_size: usize,
}

/*
/// Add quotes
fn add_quotes<S>(hash: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match hash {
        Some(string) => match string.parse::<u128>() {
            Ok(integer) if integer > 1_000_000_000_000 => {
                serializer.collect_str(&format!("'{:?}'", integer))
            }
            Ok(_) => serializer.collect_str(&string.to_string()),
            Err(_) => serializer.collect_str(&string.to_string()),
        },
        None => serializer.serialize_none(),
    }
}
*/
