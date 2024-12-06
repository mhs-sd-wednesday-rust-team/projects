use std::{error::Error, fs::File, io::BufReader};

use crate::ir::BuiltinCommand;
use clap::Parser;
use counter_scope::CounterScope;
use counters::{ByteCounter, CharacterCounter, MaxLineLengthCounter, NewlineCounter, WordCounter};
use stat_table::StatTable;
use utf8_chars::BufReadCharsExt;

mod counter_scope;
mod counters;
mod stat_table;

/// Print newline, word, and byte counts for each file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Read from files
    file: Vec<String>,

    /// Print the byte counts
    #[arg(short = 'c', long = "bytes")]
    print_byte_counts: bool,

    /// Print the character counts
    #[arg(short = 'm', long = "chars")]
    print_character_count: bool,

    /// Print the newline counts
    #[arg(short = 'l', long = "lines")]
    print_newline_count: bool,

    /// Print the word counts
    #[arg(short = 'w', long = "words")]
    print_word_count: bool,

    /// Print the length of the longest line
    #[arg(short = 'L', long = "max-line-length")]
    print_max_line_length: bool,
}

impl From<&Args> for CounterScope {
    fn from(args: &Args) -> Self {
        let mut scope = Self::default();
        if args.print_newline_count {
            scope.add_counter::<NewlineCounter>();
        }
        if args.print_word_count {
            scope.add_counter::<WordCounter>();
        }
        if args.print_character_count {
            scope.add_counter::<CharacterCounter>();
        }
        if args.print_byte_counts {
            scope.add_counter::<ByteCounter>();
        }
        if args.print_max_line_length {
            scope.add_counter::<MaxLineLengthCounter>();
        }
        if scope.is_empty() {
            scope.add_counter::<NewlineCounter>();
            scope.add_counter::<WordCounter>();
            scope.add_counter::<ByteCounter>();
        }
        scope
    }
}

/// Implements the wc built-in command.
///
/// WcCommand processes specified files and counts their contents.
/// For each file, it gathers statistics such as lines, words, and characters,
/// and outputs a summary, including a total if multiple files are specified.
#[derive(Default, Debug)]
pub struct WcCommand;

impl BuiltinCommand for WcCommand {
    fn exec(
        &self,
        args: Vec<String>,
        stdin: &mut dyn std::io::Read,
        _stderr: &mut dyn std::io::Write,
        stdout: &mut dyn std::io::Write,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut args = Args::try_parse_from(args.into_iter())?;
        let mut scope = CounterScope::from(&args);
        let mut stat_table = StatTable::default();

        if args.file.is_empty() {
            args.file.push("-".to_string());
        }

        for path in args.file.as_slice() {
            let mut _file_binding = Option::<File>::None;

            let file = match path.as_str() {
                "-" => &mut *stdin,
                path => {
                    let f = File::open(path)?;
                    _file_binding = Some(f);
                    _file_binding.as_mut().unwrap()
                }
            };
            let mut buf = BufReader::new(file);

            for ch in buf.chars().map(|c| c.unwrap()) {
                scope.count(ch);
            }

            stat_table.add_row(path.clone(), scope.reset());
        }

        if args.file.len() > 1 {
            stat_table.add_row("total".to_string(), scope.total());
        }

        writeln!(stdout, "{}", stat_table)?;
        Ok(())
    }

    fn tag(&self) -> &'static str {
        "wc"
    }
}
