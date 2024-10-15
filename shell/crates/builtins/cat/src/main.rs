use clap::Parser;
use std::{fs::File, io::Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut file = File::open(args.file)?;

    let mut buf = String::default();
    file.read_to_string(&mut buf)?;

    print!("{buf}");

    Ok(())
}
