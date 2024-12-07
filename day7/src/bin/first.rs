use std::env;
use std::fs;

fn add(x: i64, y: i64) -> i64 {
    x + y
}
fn mul(x: i64, y: i64) -> i64 {
    x * y
}

const POSSIBLE_OPERATORS: [fn(i64, i64) -> i64; 2] = [add, mul];

fn try_operators(numbers: &Vec<i64>, current_result: i64, desired_result: i64, i: usize) -> bool {
    for operator in &POSSIBLE_OPERATORS {
        let result = operator(current_result, numbers[i]);
        if i + 1 == numbers.len() {
            if result == desired_result {
                return true;
            }
        } else {
            if try_operators(numbers, result, desired_result, i + 1) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut sum = 0;
    for line in contents.lines() {
        let (result, numbers) = line.split_once(": ").unwrap();
        let numbers = numbers.split(" ").map(|x| x.parse().unwrap()).collect();
        let result = result.parse().unwrap();
        if try_operators(&numbers, numbers[0], result, 1) {
            sum += result;
        }
    }
    println!("{}", sum);
}
