use crate::{clear_terminal_screen, Algorithm, MyResult};
use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{generate, Generator, Shell};
use serde::Serialize;
use std::{fs, io, path::PathBuf, process};

#[derive(Debug, Default, Clone, ValueEnum, Serialize)]
pub enum ResultFormat {
    Json,
    Yaml,
    #[default]
    Personal,
}

// https://stackoverflow.com/questions/74068168/clap-rs-not-printing-colors-during-help
fn get_styles() -> clap::builder::Styles {
    let cyan = anstyle::Color::Ansi(anstyle::AnsiColor::Cyan);
    let green = anstyle::Color::Ansi(anstyle::AnsiColor::Green);
    let yellow = anstyle::Color::Ansi(anstyle::AnsiColor::Yellow);

    clap::builder::Styles::styled()
        .placeholder(anstyle::Style::new().fg_color(Some(yellow)))
        .usage(anstyle::Style::new().fg_color(Some(cyan)).bold())
        .header(
            anstyle::Style::new()
                .fg_color(Some(cyan))
                .bold()
                .underline(),
        )
        .literal(anstyle::Style::new().fg_color(Some(green)))
}

// https://docs.rs/clap/latest/clap/struct.Command.html#method.help_template
const APPLET_TEMPLATE: &str = "\
{before-help}
{about-with-newline}
{usage-heading} {usage}

{all-args}
{after-help}";

/// Command Line Arguments
#[derive(Parser, Debug)]
#[command(
    // Read from `Cargo.toml`
    author, version, about,
    long_about = None,
    next_line_help = true,
    help_template = APPLET_TEMPLATE,
    styles=get_styles(),
)]
pub struct Arguments {
    /// Choose the hash algorithm.
    #[arg(short('a'), long("algorithm"), value_enum, default_value_t = Algorithm::default())]
    pub algorithm: Algorithm,

    /// Set a minimum file size (in bytes) to search for identical files.
    ///
    /// keep files whose size is greater than or equal to a minimum value.
    ///
    /// size >= min_size
    #[arg(short('b'), long("min_size"), required = false, default_value_t = 0)]
    pub min_size: u64,

    /// Set a maximum file size (in bytes) to search for identical files.
    ///
    /// keep files whose size is less than or equal to a maximum value.
    ///
    /// size <= max_size
    #[arg(
        short('B'), long("max_size"),
        required = false,
        default_value_t = u64::MAX,
        hide_default_value = true,
    )]
    pub max_size: u64,

    /// Set the output directory for the CSV file (fif.csv).
    ///
    /// By default, use the current directory.
    ///
    /// That is, export identical file information to CSV format.
    ///
    /// CSV: Comma-separated Values
    ///
    /// Delimiter: ';'
    #[arg(short('c'), long("csv_dir"), required = false)]
    pub csv_dir: Option<PathBuf>,

    /// Set the minimum depth to search for identical files.
    ///
    /// depth >= min_depth
    #[arg(short('d'), long("min_depth"), required = false, default_value_t = 0)]
    pub min_depth: usize,

    /// Set the maximum depth to search for identical files.
    ///
    /// depth <= max_depth
    #[arg(
        short('D'), long("max_depth"), 
        required = false,
        default_value_t = usize::MAX,
        hide_default_value = true,
    )]
    pub max_depth: usize,

    /// Prints extended path of identical files, otherwise relative path.
    #[arg(short('e'), long("extended_path"), default_value_t = false)]
    pub extended_path: bool,

    /// Minimum frequency (number of identical files) to be filtered.
    ///
    /// If n = 1, all files will be reported.
    ///
    /// Default value = 2
    #[arg(
        short('f'), long("min_frequency"), 
        required = false,
        default_value_t = 2,
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    pub min_frequency: u64,

    /// Maximum frequency (number of identical files) to be filtered.
    #[arg(
        short('F'), long("max_frequency"), 
        required = false,
        default_value_t = u64::MAX,
        hide_default_value = true,
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    pub max_frequency: u64,

    /**
    If provided, outputs the completion file for given shell.

    ### How to generate shell completions for Z-shell:

    #### Example 1 (as a regular user):
    Generate completion_derive.zsh file with:

    ```console

        find-identical-files --generate=zsh > completion_derive.zsh

    ```

    Append the contents of the completion_derive.zsh file to the end of completion zsh file.

    ZSH completions are commonly stored in any directory listed in your `$fpath` variable.

    On Linux, view `$fpath` variable with:

    ```console

        echo $fpath | perl -nE 'say for split /\s+/'

    ```

    And then, execute:

    ```console

        compinit && zsh

    ```

    #### Example 2 (as a regular user):
    Generate completions to rustup and find-identical-files.

    Visible to only the regular user.

    ```console

        mkdir -p ~/.oh-my-zsh/functions

        rustup completions zsh > ~/.oh-my-zsh/functions/_rustup

        find-identical-files --generate=zsh > ~/.oh-my-zsh/functions/_find-identical-files

        compinit && zsh

    ```

    #### Example 3 (as root):

    Generate completions to rustup and find-identical-files.

    Visible to all system users.

    ```console

        mkdir -p /usr/local/share/zsh/site-functions

        rustup completions zsh > /usr/local/share/zsh/site-functions/_rustup

        find-identical-files --generate=zsh > /usr/local/share/zsh/site-functions/_find-identical-files

        compinit && zsh

    ```

    See `rustup completions` for detailed help.

    <https://github.com/clap-rs/clap/blob/master/clap_complete/examples/completion-derive.rs>
    */
    #[arg(short('g'), long("generate"), value_enum)]
    pub generator: Option<Shell>,

    /// Set the input directory where to search for identical files
    /// [default: current directory].
    #[arg(short('i'), long("input_dir"), required = false)]
    pub input_dir: Option<PathBuf>,

    /// Omit hidden files (starts with '.'), otherwise search all files.
    #[arg(short('o'), long("omit_hidden"), default_value_t = false)]
    pub omit_hidden: bool,

    /// Print the result in the chosen format.
    #[arg(short('r'), long("result_format"), value_enum, default_value_t = ResultFormat::default())]
    pub result_format: ResultFormat,

    /// Sort result by number of identical files, otherwise sort by file size.
    #[arg(short('s'), long("sort"), default_value_t = false)]
    pub sort: bool,

    /// Show total execution time.
    #[arg(short('t'), long("time"), default_value_t = false)]
    pub time: bool,

    /// Show intermediate runtime messages.
    #[arg(short('v'), long("verbose"), default_value_t = false)]
    pub verbose: bool,

    /// Wipe (Clear) the terminal screen before listing the identical files.
    ///
    /// On Linux, to clear use the command:
    ///
    /// tput reset
    ///
    /// Unlike the clear command, the reset command does more than just clear the terminal screen.
    ///
    /// It also resets the terminal to its default settings.
    #[arg(short('w'), long("wipe_terminal"), default_value_t = false)]
    // action = ArgAction::SetTrue
    pub wipe_terminal: bool,

    /// Set the output directory for the XLSX file (fif.xlsx).
    ///
    /// By default, use the current directory.
    ///
    /// That is, export identical file information to XLSX format.
    ///
    /// XLSX: Excel file
    #[arg(short('x'), long("xlsx_dir"), required = false)]
    pub xlsx_dir: Option<PathBuf>,
}

impl Arguments {
    /// Build Arguments struct
    pub fn build() -> MyResult<Arguments> {
        let args: Arguments = Arguments::parse();

        if let Some(generator) = args.generator {
            args.print_completions(generator);
        }

        if args.wipe_terminal {
            clear_terminal_screen();
        }

        args.validate_range_size()?;
        args.validate_range_depth()?;
        args.validate_range_frequency()?;
        args.validate_dir_path()?;

        /*
        // validate simultaneously.
        std::thread::scope(|s| {
            s.spawn(|| -> MyResult<()> {
                args.validate_range_size()?;
                Ok(())
            });

            s.spawn(|| -> MyResult<()> {
                args.validate_range_depth()?;
                Ok(())
            });

            s.spawn(|| -> MyResult<()> {
                args.validate_range_frequency()?;
                Ok(())
            });

            s.spawn(|| -> MyResult<()> {
                args.validate_dir_path()?;
                Ok(())
            });
        });
        */

        Ok(args)
    }

    /// Print shell completions to standard output
    fn print_completions<G>(&self, gen: G)
    where
        G: Generator + std::fmt::Debug,
    {
        let mut cmd = Arguments::command();
        let cmd_name = cmd.get_name().to_string();
        let mut stdout = io::stdout();

        eprintln!("Generating completion file for {gen:?}...");
        generate(gen, &mut cmd, cmd_name, &mut stdout);
        process::exit(1);
    }

    /// Get the size range (inclusive)
    ///
    /// min_size <= size <= max_size
    pub fn size_is_included(&self, size: u64) -> bool {
        size >= self.min_size && size <= self.max_size
    }

    fn validate_range_size(&self) -> MyResult<()> {
        if self.min_size > self.max_size {
            eprintln!("fn validate_range_size()");
            eprintln!("max_size cannot be less than min_size");
            eprintln!("min_size: {}", self.min_size);
            eprintln!("max_size: {}", self.max_size);
            process::exit(1);
        }

        Ok(())
    }

    fn validate_range_depth(&self) -> MyResult<()> {
        if self.min_depth > self.max_depth {
            eprintln!("fn validate_range_depth()");
            eprintln!("max_depth cannot be less than min_depth");
            eprintln!("min_depth: {}", self.min_depth);
            eprintln!("max_depth: {}", self.max_depth);
            process::exit(1);
        }

        Ok(())
    }

    fn validate_range_frequency(&self) -> MyResult<()> {
        if self.min_frequency > self.max_frequency {
            eprintln!("fn validate_range_frequency()");
            eprintln!("max_frequency cannot be less than min_frequency");
            eprintln!("min_frequency: {}", self.min_frequency);
            eprintln!("max_frequency: {}", self.max_frequency);
            process::exit(1);
        }

        Ok(())
    }

    /// Validate directory paths
    fn validate_dir_path(&self) -> MyResult<()> {
        let paths = [&self.input_dir, &self.csv_dir, &self.xlsx_dir];

        for dir_path in paths.into_iter().flatten() {
            if !std::path::Path::new(&dir_path).try_exists()? {
                eprintln!("fn validate_dir_path()");
                eprintln!("The path {dir_path:?} was not found!");
                process::exit(1);
            };

            if !dir_path.is_dir() {
                eprintln!("fn validate_dir_path()");
                eprintln!("{dir_path:?} is not a directory!");
                process::exit(1);
            }

            // Check if able to write inside directory
            let metadada = fs::metadata(dir_path)?;

            if metadada.permissions().readonly() {
                eprintln!("fn validate_dir_path()");
                eprintln!("No write permission");
                eprintln!("{dir_path:?} is readonly!");
                process::exit(1);
            }
        }

        Ok(())
    }
}
