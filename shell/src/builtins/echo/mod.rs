use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    content: Vec<String>,

    #[arg(short = 'n')]
    remove_trailing_newline: bool,
}

fn main() {
    let args = Args::parse();
    let mut output = args.content.join(" ");
    (!args.remove_trailing_newline).then(|| output += "\n");
    print!("{output}")
}
