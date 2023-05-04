fn main() {
    let number: u32 = read_line().trim().parse().unwrap();
    let factorial: u32 = factorial(number);
    println!("{}", factorial);
}

fn factorial(mut number: u32) -> u32 {
    for i in 1..number {
        number *= i;
    }

    return number;
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not get input");
    return input;
}
