# find-identical-files
Find identical files according to their size and hashing algorithm.

Therefore, a file is identical to another if they both have the same size and hash.

"A hash function is a mathematical algorithm that takes an input (in this case, a file)
and produces a fixed-size string of characters, known as a hash value or checksum.
The hash value acts as a summary representation of the original input.
This hash value is unique (disregarding unlikely [collisions](https://en.wikipedia.org/wiki/Hash_collision))
to the input data, meaning even a slight change in the input will result in a completely different hash value."

To find identical files, 3 procedures were performed:

Procedure 1. Group files by `size`.

Procedure 2. Group files by `hash(first_bytes)` with ahash algorithm.

Procedure 3. Group files by `hash(entire_file)` with chosen algorithm.

Hash algorithm options are:

1. [ahash](https://crates.io/crates/ahash) (used by [hashbrown](https://crates.io/crates/hashbrown))

2. [blake version 3](https://crates.io/crates/blake3) (default)

3. [fxhash](https://crates.io/crates/rustc-hash) ([used](https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html) by`FireFox` and `rustc`)

4. [sha256](https://github.com/RustCrypto/hashes)

5. [sha512](https://github.com/RustCrypto/hashes)

find-identical-files just reads the files and never changes their contents.
See the [open_file](https://docs.rs/find-identical-files/latest/src/find_identical_files/lib.rs.html#62-80) function to verify.

## Usage examples

### 1. To find identical files in the current directory, run the command:
```
find-identical-files
```

The number of identical files is the number of times the same file is found (number of repetitions or frequency).

By default, identical files will be filtered and those whose frequency is two (duplicates) or more will be selected.

### 2. Search files in current directory with at least N identical files, run the command:
```
find-identical-files -f N
```
such that N is an integer greater than or equal to 1 (N >= 1).

With the `-f` (or `--min_frequency`) argument option, set the minimum frequency (number of identical files).

With the `-F` (or `--max_frequency`) argument option, set the maximum frequency (number of identical files).

1. To report all files:

Useful for getting hash information for all files in the current directory.
```
find-identical-files -f 1
```

2. Look for duplicate or higher frequency files (default):
```
find-identical-files
```
or
```
find-identical-files -f 2
```

3. Look for files whose frequency is exactly 4:
```
find-identical-files -f 4 -F 4
```

### 3. To find identical files in the current directory whose size is greater than or equal to N bytes:
```
find-identical-files -b N
```
such that N is an integer (N >= 0).

With the `-b` (or `--min_size`) argument option, set the minimum size (in bytes).

With the `-B` (or `--max_size`) argument option, set the maximum size (in bytes).

1. To find identical files whose size is greater than or equal to 8 bytes:
```
find-identical-files -b 8
```

2. To find identical files whose size is less than or equal to 1024 bytes:
```
find-identical-files -B 1024
```

3. To find identical files whose size is between 8 and 1024 bytes:
```
find-identical-files -b 8 -B 1024
```

4. To find identical files whose size is exactly 1024 bytes:
```
find-identical-files -b 1024 -B 1024
```

### 4. To find identical files with `fxhash` algorithm and `yaml` format:
```
find-identical-files -twa fxhash -r yaml
```

### 5. Export identical file information from the current directory to an CSV file (fif.csv).

1. The CSV file will be saved in the currenty directory:

```
find-identical-files -c .
```
2. The CSV file will be saved in the `/tmp` directory:
```
find-identical-files -c /tmp
```
or
```
find-identical-files --csv_dir=/tmp
```

### 6. Export identical file information from the current directory to an XLSX file (fif.xlsx).

1. The XLSX file will be saved in the `~/Downloads` directory:

```
find-identical-files -x ~/Downloads
```
2. The XLSX file will be saved in the `/tmp` directory:
```
find-identical-files -x /tmp
```
or
```
find-identical-files --xlsx_dir=/tmp
```

### 7. To find identical files in the `Downloads` directory with the `ahash` algorithm, redirect the output to a `json` file (/tmp/fif.json) and export the result to an XLSX file (/tmp/fif . xlsx) for further analysis:

```
find-identical-files -tvi ~/Downloads -a ahash -r json > /tmp/fif.json -x /tmp
```

### 8. Get information using [jq](https://jqlang.github.io/jq/):

1. Print all hashes:
```
find-identical-files -r json | jq -sr '.[:-1].[].["File information"].hash'
```

2. Get information from the first identical file:
```
find-identical-files -r json | jq -s '.[0]'
```

3. Get information from the 15th identical file (if it exists):
```
find-identical-files -r json | jq -s '.[14]'
```

4. Get information from the range [a,b) with Start (a) inclusive and End (b) exclusive.

For a = 2 and b = 5:

```
find-identical-files -r json | jq -s '.[2:5]'
```

5. Get summary information:
```
find-identical-files -r json | jq -s '.[-1]'
```

Another option is to redirect the result to a temporary file and read specific information:
```
find-identical-files -vr json > /tmp/fif

jq -sr '.[:-1].[].["File information"].hash' /tmp/fif
jq -s '.[0]' /tmp/fif
jq -s '.[-2]' /tmp/fif
jq -s '.[-1]' /tmp/fif
jq -s '.[-1]["Total number of identical files"]' /tmp/fif
```

## Help

Type in the terminal `find-identical-files -h` to see the help messages and all available options:
```
find identical files according to their size and hashing algorithm

Usage: find-identical-files [OPTIONS]

Options:
  -a, --algorithm <ALGORITHM>
          Choose the hash algorithm [default: blake3] [possible values: ahash, blake3, fxhash, sha256, sha512]
  -b, --min_size <MIN_SIZE>
          Set a minimum file size (in bytes) to search for identical files [default: 0]
  -B, --max_size <MAX_SIZE>
          Set a maximum file size (in bytes) to search for identical files
  -c, --csv_dir <CSV_DIR>
          Set the output directory for the CSV file (fif.csv)
  -d, --min_depth <MIN_DEPTH>
          Set the minimum depth to search for identical files [default: 0]
  -D, --max_depth <MAX_DEPTH>
          Set the maximum depth to search for identical files
  -e, --extended_path
          Prints extended path of identical files, otherwise relative path
  -f, --min_frequency <MIN_FREQUENCY>
          Minimum frequency (number of identical files) to be filtered [default: 2]
  -F, --max_frequency <MAX_FREQUENCY>
          Maximum frequency (number of identical files) to be filtered
  -g, --generate <GENERATOR>
          If provided, outputs the completion file for given shell [possible values: bash, elvish, fish, powershell, zsh]
  -i, --input_dir <INPUT_DIR>
          Set the input directory where to search for identical files [default: current directory]
  -o, --omit_hidden
          Omit hidden files (starts with '.'), otherwise search all files
  -r, --result_format <RESULT_FORMAT>
          Print the result in the chosen format [default: personal] [possible values: json, yaml, personal]
  -s, --sort
          Sort result by number of identical files, otherwise sort by file size
  -t, --time
          Show total execution time
  -v, --verbose
          Show intermediate runtime messages
  -w, --wipe_terminal
          Wipe (Clear) the terminal screen before listing the identical files
  -x, --xlsx_dir <XLSX_DIR>
          Set the output directory for the XLSX file (fif.xlsx)
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

## Building

To build and install from source, run the following command:
```
cargo install find-identical-files
```
Another option is to install from github:
```
cargo install --git https://github.com/claudiofsr/find-identical-files.git
```

## Mutually exclusive features

### Walking a directory recursively: jwalk or walkdir.

In general, [jwalk](https://crates.io/crates/jwalk) (default)
is faster than [walkdir](https://crates.io/crates/walkdir).

But if you prefer to use walkdir:
```
cargo install --features walkdir find-identical-files
```