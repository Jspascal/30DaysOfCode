use std::io;

fn main() {
    let input = io::stdin();
    let mut keyboard_i = String::new();
    input
        .read_line(&mut keyboard_i)
        .expect("couldn't read input");
    let user_i: u16 = keyboard_i.trim().parse().expect("invalid input");
    if user_i > 100 {
        println!("number must be above 100");
        std::process::exit(0x0100);
    }
    if user_i % 2 != 0 {
        println!("Weird");
    }
    if user_i % 2 == 0 {
        if user_i >= 2 && user_i <= 5 {
            println!("Not Weird");
        }
        if user_i >= 6 && user_i <= 20 {
            println!("Weird");
        }
        if user_i > 20 {
            println!("Not Weird");
        }
    }
}
