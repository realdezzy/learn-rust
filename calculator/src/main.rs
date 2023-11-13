use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number: f32 = first.parse().unwrap();
    let second_number: f32 = second.parse().unwrap();
    let result: f32 = operate(operator, first_number, second_number);

    println!("{} {} {} = {}", first, operator, second, result);
}

fn operate(operator: char, first: f32, second: f32)  -> f32 {
    match operator {
        '+' => return first + second,
        '-' => return first - second,
        '*' | 'x' | 'X' => return first * second,
        '/' => return first / second,
        _ => panic!("Invalid operator"),
    }
}