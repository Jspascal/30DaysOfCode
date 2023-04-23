use std::io;

fn main() {
    let i = 5;
    let d = 4.0;
    let s: String = "30 Days of Code ".to_owned();
    let input = io::stdin();

    let mut keyboard_i = String::new();
    input
        .read_line(&mut keyboard_i)
        .expect("unable to read int");
    let user_i: u32 = keyboard_i.trim().parse().expect("invalid input");

    let mut keyboard_d = String::new();
    input
        .read_line(&mut keyboard_d)
        .expect("unable to read int");
    let user_d: f32 = keyboard_d.trim().parse().expect("invalid input");

    let mut keyboard_s = String::new();
    input
        .read_line(&mut keyboard_s)
        .expect("unable to read int");
    let user_s = keyboard_s;

    println!("{}", i + user_i);
    println!("{}", d + user_d);
    println!("{}", s + &user_s);
}
