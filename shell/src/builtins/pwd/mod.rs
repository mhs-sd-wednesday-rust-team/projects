use std::env::current_dir;

fn main() -> std::io::Result<()> {
    let path = current_dir()?;
    println!("{}", path.display());
    Ok(())
}
