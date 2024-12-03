use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut i = 0;
    let mut sum = 0;
    while i < contents.len() - 8 {
        if &contents[i..i + 4] == "mul(" {
            let mut j = i + 4;
            let mut number = String::new();
            let mut bad = false;
            while &contents[j..j + 1] != "," {
                if !contents[j..j + 1].chars().next().unwrap().is_numeric() {
                    bad = true;
                    break;
                }
                number += &contents[j..j + 1];
                j += 1;
                if j == contents.len() {
                    bad = true;
                    break;
                }
            }
            if bad {
                i += 1;
                continue;
            }
            let mut number1 = String::new();
            let mut bad = false;
            j += 1;
            while &contents[j..j + 1] != ")" {
                if !contents[j..j + 1].chars().next().unwrap().is_numeric() {
                    bad = true;
                    break;
                }
                number1 += &contents[j..j + 1];
                j += 1;
                if j == contents.len() {
                    bad = true;
                    break;
                }
            }
            if bad {
                i += 1;
                continue;
            }
            match number.parse::<i32>() {
                Ok(number) => match number1.parse::<i32>() {
                    Ok(number1) => sum += number * number1,
                    Err(_) => i += 1,
                },
                Err(_) => i += 1,
            }
        }
        i += 1;
    }
    println!("{}", sum);
}
