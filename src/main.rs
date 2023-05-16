use std::env::{args, Args};

fn main() {

    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_num: f32 = first.parse().unwrap();
    let second_num = second.parse::<f32>().unwrap();

    println!("{} {} {}", first_num, operator, second_num);

    let answer = operate(operator, first_num, second_num);

    println!("answer = {}", answer);

    let answerStr = output(first_num, operator, second_num, answer);
    println!("{:?}", answerStr)
}

fn operate(operator: char, first_num: f32, second_num: f32) -> f32 {
    if operator == '+' {
        return first_num + second_num;
    } else if operator == '-' {
        return first_num - second_num;
    } else if operator == '*' {
        return first_num * second_num;
    } else { return first_num / second_num };
}

fn output(first_num: f32, operator: char, second_num: f32, answer: f32) -> String {
    format!("{} {} {} = {}", first_num, operator, second_num, answer)
}
