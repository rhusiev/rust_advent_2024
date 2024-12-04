use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut count = 0;
    let expected_word = "XMAS";
    for i in 0..lines.len() {
        let line = lines[i];
        for j in 0..line.len() {
            let mut found_l = true;
            let mut found_r = true;
            let mut found_u = true;
            let mut found_d = true;
            let mut found_lu = true;
            let mut found_ru = true;
            let mut found_ld = true;
            let mut found_rd = true;
            for dist in 0..4 {
                if j + dist >= line.len()
                    || line.chars().nth(j + dist).unwrap()
                        != expected_word.chars().nth(dist).unwrap()
                {
                    found_r = false;
                }
                if dist > j
                    || line.chars().nth(j - dist).unwrap()
                        != expected_word.chars().nth(dist).unwrap()
                {
                    found_l = false;
                }
                if i + dist >= lines.len()
                    || lines[i + dist].chars().nth(j).unwrap()
                        != expected_word.chars().nth(dist).unwrap()
                {
                    found_d = false;
                }
                if dist > i
                    || lines[i - dist].chars().nth(j).unwrap()
                        != expected_word.chars().nth(dist).unwrap()
                {
                    found_u = false;
                }
                if j + dist >= line.len()
                    || i + dist >= lines.len()
                    || lines[i + dist].chars().nth(j + dist).unwrap()
                        != expected_word.chars().nth(dist).unwrap()
                {
                    found_rd = false;
                }
                if dist > j
                    || i + dist >= lines.len()
                    || lines[i + dist].chars().nth(j - dist).unwrap()
                        != expected_word.chars().nth(dist).unwrap()
                {
                    found_ld = false;
                }
                if j + dist >= line.len()
                    || dist > i
                    || lines[i - dist].chars().nth(j + dist).unwrap()
                        != expected_word.chars().nth(dist).unwrap()
                {
                    found_ru = false;
                }
                if dist > j
                    || dist > i
                    || lines[i - dist].chars().nth(j - dist).unwrap()
                        != expected_word.chars().nth(dist).unwrap()
                {
                    found_lu = false;
                }
            }
            for i in [
                found_l, found_r, found_u, found_d, found_lu, found_ru, found_ld, found_rd,
            ] {
                if i {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
