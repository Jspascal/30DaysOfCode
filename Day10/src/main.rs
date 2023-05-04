fn main() {
    let val: u32 = read_line().trim().parse().unwrap();
    let binary_number: Vec<u8> = convert_to_binary(val);
    let long_streak: u8 = longest_streak(binary_number);
    println!("{}", long_streak);
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    return input;
}

fn convert_to_binary(mut val: u32) -> Vec<u8> {
    let mut binary_number = Vec::new();
    while val > 1 {
        let i: u8 = (val % 2) as u8;
        val = val / 2;
        binary_number.push(i);
        if val == 1 {
            binary_number.push(1);
        }
    }

    return binary_number;
}

fn longest_streak(binary_number: Vec<u8>) -> u8 {
    let mut tmp: u8 = 0;
    let mut streak: u8 = 0;
    for i in 0..binary_number.len() {
        if binary_number[i] == 1 {
            tmp += 1;
        }
        if binary_number[i] == 0 && tmp >= streak {
            streak = tmp;
            tmp = 0;
        }
        if binary_number[i] == 0 && tmp < streak {
            tmp = 0;
        }
    }
    if tmp > streak {
        streak = tmp;
    }
    return streak;
}
