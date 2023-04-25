use std::io;

fn main() {
    let input = io::stdin();

    println!("Enter the price for this article");
    let mut keyboard_cost = String::new();
    input
        .read_line(&mut keyboard_cost)
        .expect("unable to read float");
    let user_cost: f32 = keyboard_cost.trim().parse().expect("invalid input");

    println!("Enter the tip percentage charged");
    let mut keyboard_tip = String::new();
    input
        .read_line(&mut keyboard_tip)
        .expect("unable to read float");
    let user_tip: f32 = keyboard_tip.trim().parse().expect("invalid input");

    println!("Enter the tax percentage");
    let mut keyboard_tax = String::new();
    input
        .read_line(&mut keyboard_tax)
        .expect("unable to read float");
    let user_tax: f32 = keyboard_tax.trim().parse().expect("invalid input");

    let total_cost: i32 = solve(user_cost, user_tip, user_tax).round() as i32;
    println!("{}", total_cost);
}

fn solve(meal_cost: f32, tip_percent: f32, tax_percent: f32) -> f32 {
    let tip = (meal_cost * tip_percent) / 100.0;
    let tax = (meal_cost * tax_percent) / 100.0;
    return meal_cost + tip + tax;
}
