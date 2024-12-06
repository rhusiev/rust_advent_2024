use std::env;
use std::fs;

fn rotate_right(direction: (i32, i32)) -> (i32, i32) {
    if direction == (0, 1) {
        (-1, 0)
    } else if direction == (-1, 0) {
        (0, -1)
    } else if direction == (0, -1) {
        (1, 0)
    } else {
        (0, 1)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut x = lines[0].len() as i32;
    let mut y = lines.len() as i32;
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                x = j as i32;
                y = i as i32;
                break;
            }
        }
        if x != line.len() as i32 {
            break;
        }
    }

    let mut direction: (i32, i32) = (0, -1);
    while 0 <= x && x < lines[0].len() as i32 && 0 <= y && y < lines.len() as i32 {
        lines[y as usize].replace_range(x as usize..x as usize + 1, "X");
        if x + direction.0 < 0
            || x + direction.0 >= lines[0].len() as i32
            || y + direction.1 < 0
            || y + direction.1 >= lines.len() as i32
        {
            break;
        }
        while lines[(y + direction.1) as usize]
            .chars()
            .nth((x + direction.0) as usize)
            == Some('#')
        {
            direction = rotate_right(direction);
        }
        x += direction.0;
        y += direction.1;
    }
    println!(
        "{}",
        lines
            .iter()
            .map(|s| s.chars().filter(|c| *c == 'X').count())
            .sum::<usize>()
    );
}
