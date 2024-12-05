use std::collections::HashMap;
use std::env;
use std::fs;

fn get_ordered_numbers(ordering: &HashMap<i32, Vec<i32>>, numbers: &Vec<i32>) -> Vec<i32> {
    let mut is_ordered = true;
    for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            if ordering.contains_key(&numbers[j]) && ordering[&numbers[j]].contains(&numbers[i]) {
                is_ordered = false;
            }
        }
    }
    if is_ordered {
        return Vec::new();
    }

    let mut necessary_ordering: HashMap<i32, Vec<i32>> = HashMap::new();
    for (key, value) in ordering {
        if !numbers.contains(key) {
            continue;
        }
        for value in value {
            if !numbers.contains(value) {
                continue;
            }
            necessary_ordering
                .entry(*key)
                .or_insert(Vec::new())
                .push(*value);
        }
    }
    let mut no_incoming: Vec<i32> = Vec::new();
    for number in numbers {
        let mut valid = true;
        for (_, value) in &necessary_ordering {
            if value.contains(number) {
                valid = false;
                break;
            }
        }
        if !valid {
            continue;
        }
        no_incoming.push(*number);
    }

    let mut result: Vec<i32> = Vec::new();
    while no_incoming.len() > 0 {
        let cur = no_incoming.pop().unwrap();
        result.push(cur);
        let mut removed = 0;
        if !necessary_ordering.contains_key(&cur) {
            continue;
        }
        for i in 0..necessary_ordering[&cur].len() {
            let next = necessary_ordering[&cur][i - removed];
            necessary_ordering.get_mut(&cur).unwrap().remove(i - removed);
            removed += 1;
            let mut incoming_into_i = 0;
            for (_, value) in &necessary_ordering {
                if value.contains(&next) {
                    incoming_into_i += 1;
                }
            }
            if incoming_into_i == 0 {
                no_incoming.push(next);
            }
        }
    }

    result
}

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
        let numbers = get_ordered_numbers(&ordering, &numbers);
        if numbers.len() == 0 {
            i += 1;
            continue;
        }
        let middle_num = numbers[numbers.len() / 2];
        sum += middle_num;
        i += 1;
    }
    println!("{}", sum);
}
