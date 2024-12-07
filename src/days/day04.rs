use regex::Regex;
use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("input/day04.txt").expect("Failed to read file.");

    solve(&input);
    solve2(&input);
}

fn solve(input: &String) {
    let input_rows = input.trim().split("\n");

    let mut character_grid: Vec<Vec<char>> = Vec::new();

    let mut xmas_count: usize = 0;

    let xmas_re = Regex::new("XMAS").unwrap();
    let samx_re = Regex::new("SAMX").unwrap();

    for row in input_rows {
        character_grid.push(row.chars().collect());
    }

    for y in 0..character_grid.len() {
        let mut row_string: String = String::new();

        for x in 0..character_grid[0].len() {
            row_string.push(character_grid[y][x]);
        }

        xmas_count += xmas_re.find_iter(&row_string).count();
        xmas_count += samx_re.find_iter(&row_string).count();
    }

    for x in 0..character_grid[0].len() {
        let mut column_string: String = String::new();

        for y in 0..character_grid.len() {
            column_string.push(character_grid[y][x]);
        }

        xmas_count += xmas_re.find_iter(&column_string).count();
        xmas_count += samx_re.find_iter(&column_string).count();
    }

    let mut start_x: usize = 0;
    let mut start_y: usize = character_grid.len();

    loop {
        let mut diagonal_down_string: String = String::new();

        let mut x = start_x;
        let mut y = start_y;

        while (x < character_grid[0].len()) & (y < character_grid.len()) {
            diagonal_down_string.push(character_grid[y][x]);

            x += 1;
            y += 1;
        }

        xmas_count += xmas_re.find_iter(&diagonal_down_string).count();
        xmas_count += samx_re.find_iter(&diagonal_down_string).count();

        if start_y > 0 {
            start_y -= 1;
        } else if start_x < character_grid[0].len() - 1 {
            start_x += 1;
        } else {
            break;
        }
    }

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;

    loop {
        let mut diagonal_up_string: String = String::new();

        let mut x = start_x;
        let mut y = start_y;

        loop {
            diagonal_up_string.push(character_grid[y][x]);

            if (x < character_grid[0].len() - 1) & (y > 0) {
                x += 1;
                y -= 1;
            } else {
                break;
            }
        }

        xmas_count += xmas_re.find_iter(&diagonal_up_string).count();
        xmas_count += samx_re.find_iter(&diagonal_up_string).count();

        if start_y < character_grid.len() - 1 {
            start_y += 1;
        } else if start_x < character_grid[0].len() - 1 {
            start_x += 1;
        } else {
            break;
        }
    }

    println!("XMAS count 1: {}", xmas_count);
}

fn solve2(input: &String) {
    let input_rows = input.trim().split("\n");

    let mut character_grid: Vec<Vec<char>> = Vec::new();

    for row in input_rows {
        character_grid.push(row.chars().collect());
    }

    let mut xmas_count: usize = 0;

    for y in 0..(character_grid.len() - 2) {
        for x in 0..(character_grid[0].len() - 2) {
            let mut mas1 = String::new();
            let mut mas2 = String::new();

            mas1.push(character_grid[y][x]);
            mas1.push(character_grid[y + 1][x + 1]);
            mas1.push(character_grid[y + 2][x + 2]);

            mas2.push(character_grid[y][x + 2]);
            mas2.push(character_grid[y + 1][x + 1]);
            mas2.push(character_grid[y + 2][x]);

            if (mas1 == "MAS" || mas1 == "SAM") & (mas2 == "MAS" || mas2 == "SAM") {
                xmas_count += 1;
            }
        }
    }

    println!("XMAS count 2: {}", xmas_count);
}
