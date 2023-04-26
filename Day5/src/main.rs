use std::io;

fn main() {
    let mut keyboard_input = String::new();
    let input = io::stdin();
    input
        .read_line(&mut keyboard_input)
        .expect("could not read input");
    let keyboard_int: i32 = keyboard_input.trim().parse().expect("invalid input type");

    for i in 1..11 {
        let resutl: i32 = keyboard_int * i;
        println!("{} x {} = {}", keyboard_int, i, resutl);
    }
}
