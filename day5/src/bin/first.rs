use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut ordering: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut i = 0;
    while i < contents.lines().count() {
        let line = contents.lines().nth(i).unwrap();
        if line == "" {
            i += 1;
            break;
        }
        let (first, after_first) = line.split_once("|").unwrap();
        let first: i32 = first.trim().parse().unwrap();
        let after_first: i32 = after_first.trim().parse().unwrap();
        ordering
            .entry(first)
            .or_insert(Vec::new())
            .push(after_first);
        i += 1;
    }

    let mut sum = 0;
    while i < contents.lines().count() {
        let line = contents.lines().nth(i).unwrap();
        let numbers = line
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        let mut valid = true;
        for j in 0..numbers.len() {
            let mut k = j + 1;
            while k < numbers.len() {
                if ordering
                    .get(&numbers[k])
                    .unwrap_or(&Vec::new())
                    .contains(&numbers[j])
                {
                    valid = false;
                    break;
                }
                k += 1;
            }
            if !valid {
                break;
            }
        }
        if !valid {
            i += 1;
            continue;
        }
        let middle_num = numbers[numbers.len() / 2];
        sum += middle_num;
        i += 1;
    }
    println!("{}", sum);
}
