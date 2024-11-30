use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
};

use regex::{RegexSet, RegexSetBuilder};

use crate::ir::BuiltinCommand;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    patterns: String,

    files: Vec<String>,

    #[arg(short = 'w', long = "word-regexp")]
    word_regexp: bool,

    #[arg(short = 'i', long = "ignore-case")]
    ignore_case: bool,

    #[arg(short = 'A', long = "after-context")]
    after_context: Option<usize>,
}

/// Implements grep built-in command
///
/// Usage: grep [OPTION...] PATTERNS [FILE...]
/// TODO: Description
#[derive(Default, Debug)]
pub struct GrepCommand {}

pub struct GrepFlags {
    word_regexp: bool,
    ignore_case: bool,
    after_context: usize,
}

impl GrepCommand {
    const STDIN_WILDCARD: &str = "-";
    const MATCH_GROUP_DELIM: &str = "--";

    fn parse_patterns(
        raw_patterns: &str,
        delim: &str,
        flags: &GrepFlags,
    ) -> Result<RegexSet, regex::Error> {
        let patterns: Vec<String> = raw_patterns
            .split(delim)
            .filter(|s| !s.is_empty())
            .map(|pattern| {
                if flags.word_regexp {
                    format!(r"\b{}\b", regex::escape(pattern))
                } else {
                    regex::escape(pattern)
                }
            })
            .collect();

        RegexSetBuilder::new(patterns)
            .case_insensitive(flags.ignore_case)
            .build()
    }

    fn grep_from_source(
        source: &mut dyn Read,
        patterns: &RegexSet,
        flags: &GrepFlags,
    ) -> Result<Vec<Vec<String>>, Box<dyn Error + Sync + Send>> {
        let mut buffer = BufReader::new(source);
        let mut matched_line_groups: Vec<Vec<String>> = Vec::new();

        let mut line = String::new();
        let mut context_count = 0;
        let mut has_active_group = false;
        matched_line_groups.push(Vec::new());

        while buffer.read_line(&mut line)? > 0 {
            if patterns.is_match(&line) {
                matched_line_groups
                    .last_mut()
                    .unwrap()
                    .push(line.trim().to_string());
                context_count = flags.after_context;
                has_active_group = context_count > 0;
            } else if context_count > 0 {
                matched_line_groups
                    .last_mut()
                    .unwrap()
                    .push(line.trim().to_string());
                context_count -= 1;
            } else if has_active_group {
                matched_line_groups.push(Vec::new());
                has_active_group = false;
            }
            line.clear();
        }

        Ok(matched_line_groups)
    }

    fn format_match_report(match_report: &HashMap<String, Vec<Vec<String>>>) -> String {
        let multiple_files = match_report.len() > 1;

        match_report
            .iter()
            .flat_map(|(filename, match_groups)| {
                match_groups
                    .iter()
                    .map(|match_group| {
                        match_group
                            .iter()
                            .map(|line| {
                                if multiple_files {
                                    format!("{filename}:{line}")
                                } else {
                                    line.to_string()
                                }
                            })
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<Vec<String>>>()
            })
            .map(|group| group.join("\n"))
            .collect::<Vec<String>>()
            .join(format!("\n{}\n", Self::MATCH_GROUP_DELIM).as_str())
    }
}

impl BuiltinCommand for GrepCommand {
    fn exec(
        &self,
        args: Vec<String>,
        stdin: &mut dyn Read,
        _stderr: &mut dyn Write,
        stdout: &mut dyn Write,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let args = Args::try_parse_from(args.into_iter())?;

        let grep_flags = GrepFlags {
            word_regexp: args.word_regexp,
            ignore_case: args.ignore_case,
            after_context: args.after_context.unwrap_or(0),
        };
        let patterns = Self::parse_patterns(&args.patterns, "\n", &grep_flags)?;

        let mut files = args.files;
        if files.is_empty() {
            files.push(String::from(Self::STDIN_WILDCARD));
        }

        let mut match_report: HashMap<String, Vec<Vec<String>>> = HashMap::default();
        for file_name in files {
            let matched_lines = if file_name == Self::STDIN_WILDCARD {
                Self::grep_from_source(stdin, &patterns, &grep_flags)?
            } else {
                let mut file = File::open(&file_name)?;
                Self::grep_from_source(&mut file, &patterns, &grep_flags)?
            };
            match_report.insert(file_name, matched_lines);
        }

        let match_report_str = Self::format_match_report(&match_report);
        stdout.write_all(match_report_str.as_bytes())?;
        writeln!(stdout)?;
        Ok(())
    }

    fn tag(&self) -> &'static str {
        "grep"
    }
}
