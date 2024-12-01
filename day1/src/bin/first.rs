use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(args[1].clone()).expect("Error reading file");

    let lines: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let mut numbers = line.split("   ");
            return (
                numbers.next().unwrap().parse::<i32>().unwrap(),
                numbers.next().unwrap().parse::<i32>().unwrap(),
            );
        })
        .collect();

    let mut first = lines.iter().map(|line| line.0).collect::<Vec<i32>>();
    let mut second = lines.iter().map(|line| line.1).collect::<Vec<i32>>();

    let mut diff_sum = 0;

    while first.len() > 0 {
        let minimal_first = *first.iter().min().unwrap();
        if let Some(index) = first.iter().position(|value| *value == minimal_first) {
            first.remove(index);
        }
        let minimal_second = *second.iter().min().unwrap();
        if let Some(index) = second.iter().position(|value| *value == minimal_second) {
            second.remove(index);
        }
        diff_sum += (minimal_first - minimal_second).abs();
    }

    println!("{}", diff_sum);
}
