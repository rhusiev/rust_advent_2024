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

fn has_loop(lines: &Vec<String>) -> bool {
    let mut x = lines[0].len() as i32;
    let mut y = lines.len() as i32;
    let height = lines.len();
    let width = lines[0].len();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                x = j as i32;
                y = i as i32;
                break;
            }
        }
        if x != width as i32 {
            break;
        }
    }

    let mut visited: Vec<Vec<(bool, i32, i32)>> = Vec::new();
    for i in 0..height {
        visited.push(Vec::new());
        for _ in 0..width {
            visited[i].push((false, 0, 0));
        }
    }

    let mut direction: (i32, i32) = (0, -1);
    while 0 <= x && x < width as i32 && 0 <= y && y < height as i32 {
        if x + direction.0 < 0
            || x + direction.0 >= width as i32
            || y + direction.1 < 0
            || y + direction.1 >= height as i32
        {
            break;
        }
        let in_direction = direction.clone();
        while lines[(y + direction.1) as usize]
            .chars()
            .nth((x + direction.0) as usize)
            == Some('#')
        {
            if visited[y as usize][x as usize].0 == true
                && visited[y as usize][x as usize].1 == direction.0
                && visited[y as usize][x as usize].2 == direction.1
            {
                // for (i, line) in lines.iter().enumerate() {
                //     let mut line = line.clone();
                //     for j in 0..width {
                //         if visited[i][j].0 {
                //             line.replace_range(j..j + 1, "X");
                //         }
                //     }
                //     println!("{}", line);
                // }
                // println!();
                return true;
            }
            direction = rotate_right(direction);
        }
        visited[y as usize][x as usize].0 = true;
        visited[y as usize][x as usize].1 = in_direction.0;
        visited[y as usize][x as usize].2 = in_direction.1;
        x += direction.0;
        y += direction.1;
    }

    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let lines_copy = lines.clone();
    let mut x = lines[0].len() as i32;
    let mut y = lines.len() as i32;
    let height = lines.len();
    let width = lines[0].len();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                x = j as i32;
                y = i as i32;
                break;
            }
        }
        if x != width as i32 {
            break;
        }
    }

    let mut direction: (i32, i32) = (0, -1);
    while 0 <= x && x < width as i32 && 0 <= y && y < height as i32 {
        lines[y as usize].replace_range(x as usize..x as usize + 1, "X");
        if x + direction.0 < 0
            || x + direction.0 >= width as i32
            || y + direction.1 < 0
            || y + direction.1 >= height as i32
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

    let mut positions_to_cycles: Vec<(usize, usize)> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            if lines_copy[i].chars().nth(j) == Some('^') {
                continue;
            }
            let mut lines = lines_copy.clone();
            lines[i].replace_range(j..j + 1, "#");
            if has_loop(&lines) {
                positions_to_cycles.push((i, j));
            }
        }
    }
    println!("{}", positions_to_cycles.len());
}
