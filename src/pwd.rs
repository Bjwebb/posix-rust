use std::env;

// Does not yet implement any options:
// https://pubs.opengroup.org/onlinepubs/9699919799/utilities/pwd.html

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}
