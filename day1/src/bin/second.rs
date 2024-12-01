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

    let first = lines.iter().map(|line| line.0).collect::<Vec<i32>>();
    let second = lines.iter().map(|line| line.1).collect::<Vec<i32>>();

    let mut similarity = 0;

    for i in first.iter() {
        let count_i_in_second = second.iter().filter(|&j| j == i).count();
        similarity += i * count_i_in_second as i32;
    }

    println!("{}", similarity);
}
