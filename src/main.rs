use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first: f64 = first.trim().parse().unwrap();
    let second: f64 = second.trim().parse().unwrap();

    let result = operate(operator, first, second);

    println!("{}", output(first, operator, second, result));
}

fn operate(operator: char, first: f64, second: f64) -> f64 {
    match operator {
        // Usando o 'match' com a var 'operator' como o atributo a ser 'matched' ou verificado
        '+' => first + second,
        '-' => first - second,
        '/' => first / second,
        '%' => first % second,
        '*' | 'x' | 'X' => first * second, // Colacando opções para o simbolo '*'. Tentar passar o operador como '\*'.
        _ => panic!("Used invalid operator. please use '+', '-', '/', '%' and '*' or 'x'"),
    }
}

fn output(first: f64, operator: char, second: f64, result: f64) -> String {
    format!("{} {} {} = {}", first, operator, second, result)
}
