use std::{error::Error, fs::File, io::BufReader};

use crate::backend::ExitStatus;
use clap::Parser;
use counter_scope::CounterScope;
use counters::{ByteCounter, CharacterCounter, MaxLineLengthCounter, NewlineCounter, WordCounter};
use stat_table::StatTable;
use utf8_chars::BufReadCharsExt;

use super::BuiltinCommand;

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

pub struct WcCommand;

impl BuiltinCommand for WcCommand {
    fn exec(args: Vec<String>) -> Result<ExitStatus, Box<dyn Error>> {
        let args = Args::try_parse_from(args.into_iter())?;
        let mut scope = CounterScope::from(&args);
        let mut stat_table = StatTable::default();

        for path in args.file.as_slice() {
            let file = File::open(&path)?;
            let mut buf = BufReader::new(file);

            for ch in buf.chars().map(|c| c.unwrap()) {
                scope.count(ch);
            }

            stat_table.add_row(path.clone(), scope.reset());
        }

        if args.file.len() > 1 {
            stat_table.add_row("total".to_string(), scope.total());
        }

        println!("{}", stat_table);

        Ok(ExitStatus::default())
    }
}
