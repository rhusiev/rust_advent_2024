use std::env;
use std::fs;

fn is_safe_my(numbers: Vec<i32>, increases: bool, decreases: bool) -> bool {
    for i in 0..numbers.len() - 1 {
        if (increases && (numbers[i] >= numbers[i + 1]))
            || (decreases && (numbers[i] <= numbers[i + 1]))
        {
            return false;
        }
        let abs = (numbers[i] - numbers[i + 1]).abs();
        if (1 > abs) || (abs > 3) {
            return false;
        }
    }
    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut safe = 0;
    for line in contents.lines() {
        let numbers = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        for i in 0..numbers.len() {
            let numbers_without_i = &[&numbers[0..i], &numbers[i + 1..]].concat();
            if is_safe_my(numbers_without_i.to_vec(), true, false)
                || is_safe_my(numbers_without_i.to_vec(), false, true)
            {
                safe += 1;
                break;
            }
        }
    }
    println!("{}", safe);
}
