use std::collections::HashMap;

fn main() {
    let T: i32 = read_line().trim().parse().unwrap();
    let mut contacts = HashMap::new();

    for _ in 0..T {
        let contact: String = read_line().trim().parse().unwrap();
        let contact_vector: Vec<String> = contact
            .split_whitespace()
            .map(|info| info.parse().expect("invalid input"))
            .collect();
        contacts.insert(contact_vector[0].to_string(), contact_vector[1].to_string());
    }

    for _ in 0..T {
        let contact_name: String = read_line().trim().parse().unwrap();
        if contacts.contains_key(&contact_name) {
            let phonenumber = contacts.get(&contact_name).unwrap();
            println!("{}={}", contact_name, phonenumber);
        } else {
            println!("Not found");
        }
    }
    std::process::exit(0x0100);
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could read user input");
    return input;
}
