use ahash::AHasher;
use blake3::Hasher as Blake3Hasher;
use clap::ValueEnum;
use rustc_hash::FxHasher;
use serde::Serialize;
use sha2::{Digest, Sha256, Sha512};
use std::{
    fmt,
    fs::File,
    hash::Hasher,
    io::{BufReader, Read},
    path::PathBuf,
};

use crate::{open_file, Arguments, MyResult};

const FIRST_BYTES: usize = 1024;
const BUFFER_SIZE: usize = 64 * 1024;
const HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub trait SliceExtension {
    fn to_hex_string(self) -> String;
}

impl SliceExtension for &[u8] {
    fn to_hex_string(self) -> String {
        self.iter()
            .flat_map(|byte| {
                let a: char = HEX[(*byte as usize) / 16];
                let b: char = HEX[(*byte as usize) % 16];
                [a, b]
            })
            .collect()
    }
}

pub trait PathBufExtension {
    fn get_hash(&self, arguments: &Arguments, procedure: u8) -> MyResult<Option<String>>;
}

impl PathBufExtension for PathBuf {
    /// Hash the first few bytes or the entire file.
    ///
    /// <https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html>
    fn get_hash(&self, arguments: &Arguments, procedure: u8) -> MyResult<Option<String>> {
        let mut file: File = open_file(self)?;

        let hash: String = if procedure == 3 {
            // Apply the chosen hash algorithm to the entire file.
            arguments.algorithm.calculate_hash(file)?
        } else {
            // Apply the hash algorithm the first bytes of the file.
            let mut buffer = [0_u8; FIRST_BYTES];

            // Read up to FIRST_BYTES bytes.
            let count = file.read(&mut buffer)?;

            let mut hasher = AHasher::default();
            hasher.write(&buffer[..count]);
            hasher.finish().to_string()

            //buffer[..count].to_hex_string()
        };

        Ok(Some(hash))
    }
}

// An attribute #[default], usable on enum unit variants, is introduced
// thereby allowing some enums to work with #[derive(Default)].
// <https://rust-lang.github.io/rfcs/3107-derive-default-enum.html>
// <https://serde.rs/attr-rename.html>
/// Hash Algorithm
#[derive(Debug, Default, Clone, Copy, ValueEnum, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum Algorithm {
    Ahash,
    #[default]
    Blake3,
    Fxhash,
    SHA256,
    SHA512,
}

/// Display an enum Algorithm in serde PascalCase.
///
/// Rename all the fields according to the given case convention.
///
/// <https://docs.rs/serde/latest/serde/ser/trait.Serializer.html#method.collect_str>
///
/// <https://serde.rs/container-attrs.html>
impl fmt::Display for Algorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // Display an enum in lowercase
        // let field: String = format!("{self:?}");
        // write!(formatter, "{}", field.to_lowercase())

        self.serialize(formatter)
    }
}

impl Algorithm {
    /// Calculate file hash using some algorithm
    pub fn calculate_hash(&self, file: File) -> MyResult<String> {
        let reader: BufReader<File> = BufReader::with_capacity(BUFFER_SIZE, file);

        match self {
            Algorithm::Ahash => get_ahash(reader),
            Algorithm::Blake3 => get_blake3(reader),
            Algorithm::Fxhash => get_fxhash(reader),
            Algorithm::SHA256 => get_sha256(reader),
            Algorithm::SHA512 => get_sha512(reader),
        }
    }
}

/// Calculates the aHash from Path.
///
/// <https://crates.io/crates/ahash>
fn get_ahash(mut reader: impl Read) -> MyResult<String> {
    let mut buffer = [0_u8; BUFFER_SIZE];
    let mut hasher = AHasher::default();

    loop {
        // read up to BUFFER_SIZE bytes to buffer
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.write(&buffer[..count]);
    }

    Ok(hasher.finish().to_string())
}

/// Calculates the Blake3 hash from Path.
///
/// <https://docs.rs/blake3/latest/blake3>
fn get_blake3<R>(mut reader: R) -> MyResult<String>
where
    R: Read,
{
    let mut buffer = [0_u8; BUFFER_SIZE];
    let mut hasher = Blake3Hasher::new();

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(hasher.finalize().to_string())
}

/// Calculates the FxHash from Path.
///
/// Fast, non-cryptographic hash function used by rustc and Firefox.
///
/// (Indeed, the Fx is short for “Firefox”.)
///
/// <https://crates.io/crates/rustc-hash>
///
/// <https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html>
fn get_fxhash<R>(mut reader: R) -> MyResult<String>
where
    R: Read,
{
    let mut buffer = [0_u8; BUFFER_SIZE];
    let mut hasher = FxHasher::default();

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.write(&buffer[..count]);
    }

    Ok(hasher.finish().to_string())
}

/// Calculates the SHA2 256 from Path.
///
/// Verify with
///
/// openssl dgst -sha256 Some_File
///
/// <https://github.com/RustCrypto/hashes/tree/master/sha2>
fn get_sha256<R>(mut reader: R) -> MyResult<String>
where
    R: Read,
{
    let mut buffer = [0_u8; BUFFER_SIZE];
    let mut hasher = Sha256::new();

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
        hasher.update(&buffer[..count]);
    }

    // Note that calling `finalize()` consumes hasher
    Ok(hasher.finalize().to_hex_string())
}

/// Calculates the SHA2 512 from Path.
///
/// Verify with
///
/// openssl dgst -sha512 Some_File
///
/// <https://github.com/RustCrypto/hashes/tree/master/sha2>
fn get_sha512<R>(mut reader: R) -> MyResult<String>
where
    R: Read,
{
    let mut buffer = [0_u8; BUFFER_SIZE];
    let mut hasher = Sha512::new();

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
        hasher.update(&buffer[..count]);
    }

    // Note that calling `finalize()` consumes hasher
    Ok(hasher.finalize().to_hex_string())
}
