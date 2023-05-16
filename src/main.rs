use std::env::{args, Args};

fn main() {

    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_num = first.parse::<f32>().unwrap();
    let second_num = second.parse::<f32>().unwrap();

    println!("{} {} {}", first_num, operator, second_num);

    let answer = operate(operator, first_num, second_num);

    println!("answer = {}", answer);

    let answer_str = output(first_num, operator, second_num, answer);
    println!("{:?}", answer_str)
}

fn operate(operator: char, first_num: f32, second_num: f32) -> f32 {
    match operator {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '/' => first_num / second_num,
        '*' | 'x' | 'X' => first_num * second_num,
        _ => panic!("invalid operator")
    }
}

fn output(first_num: f32, operator: char, second_num: f32, answer: f32) -> String {
    format!("{} {} {} = {}", first_num, operator, second_num, answer)
}
