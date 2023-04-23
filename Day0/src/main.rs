use std::io;

fn main() -> io::Result<()> {
    let mut value = String::new();
    let input = io::stdin();
    input.read_line(&mut value).expect("unable to read!");

    println!("Hello, world!");
    println!("{}", value);

    Ok(())
}
