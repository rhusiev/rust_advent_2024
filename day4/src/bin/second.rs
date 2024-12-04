use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut count = 0;
    let expected_word = "MAS";
    for i in 0..lines.len() - 2 {
        let line = lines[i];
        for j in 0..line.len() - 2 {
            let square = vec![
                &line[j..j + 3],
                &lines[i + 1][j..j + 3],
                &lines[i + 2][j..j + 3],
            ];
            let mut found_dr = true;
            let mut found_ur = true;
            let mut found_dl = true;
            let mut found_ul = true;
            for dist in 0..3 {
                if square[dist].chars().nth(dist).unwrap()
                    != expected_word.chars().nth(dist).unwrap()
                {
                    found_dr = false;
                }
                if square[dist].chars().nth(2 - dist).unwrap()
                    != expected_word.chars().nth(dist).unwrap()
                {
                    found_dl = false;
                }
                if square[2 - dist].chars().nth(dist).unwrap()
                    != expected_word.chars().nth(dist).unwrap()
                {
                    found_ur = false;
                }
                if square[2 - dist].chars().nth(2 - dist).unwrap()
                    != expected_word.chars().nth(dist).unwrap()
                {
                    found_ul = false;
                }
            }
            if !(found_dr ^ found_ur ^ found_dl ^ found_ul) && (found_dr || found_ur || found_dl || found_ul) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
