use std::env;
use std::fs;

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
        if numbers[0] == numbers[1] {
            continue;
        }
        let increases = numbers[0] < numbers[1];
        let decreases = numbers[0] > numbers[1];
        let mut this_safe = true;
        for i in 0..numbers.len() - 1 {
            if (increases && (numbers[i] >= numbers[i + 1]))
                || (decreases && (numbers[i] <= numbers[i + 1]))
            {
                this_safe = false;
                break;
            }
            let abs = (numbers[i] - numbers[i + 1]).abs();
            if (1 > abs) || (abs > 3) {
                this_safe = false;
                break;
            }
        }
        if this_safe {
            safe += 1;
        }
    }
    println!("{}", safe);
}
