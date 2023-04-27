fn main() {
    let n: usize = read_line().trim().parse().unwrap();
    let user_string: String = read_line().trim().parse().expect("invalid input type");
    let mut int_array: Vec<i32> = user_string
        .split_whitespace()
        .map(|n| n.parse().expect("invalid input"))
        .collect();
    if int_array.len() < n {
        println!("invalid input size");
        std::process::exit(0x0100);
    }
    int_array.resize(n, 0);
    int_array.reverse();

    let mut reversed_string = String::new();
    let mut cur: i8 = 0;

    for i in int_array {
        if cur == 0 {
            reversed_string = format!("{}", i);
        }
        if cur > 0 {
            reversed_string = format!("{} {}", reversed_string, i);
        }
        cur += 1;
    }

    println!("{}", reversed_string);
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read user input");
    return input;
}
