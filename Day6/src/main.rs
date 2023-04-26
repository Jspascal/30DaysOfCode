#![allow(non_snake_case)]

fn main() {
    let T: i32 = read_line().trim().parse().unwrap();

    for _ in 0..T {
        let indexedString: String = read_line().trim().parse().expect("invalid value");
        let char_vector: Vec<char> = indexedString.chars().collect();

        let mut evenString = String::new();
        let mut oddString = String::new();
        let mut i: u16 = 0;

        for c in char_vector {
            if i == 0 || i % 2 == 0 {
                evenString.push(c);
            }
            if i % 2 != 0 {
                oddString.push(c);
            }
            i += 1;
        }

        println!("{} {}", evenString, oddString);
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    return input;
}
