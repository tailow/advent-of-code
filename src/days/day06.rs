use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("input/day06.txt").expect("Failed to read file.");

    solve(&input);
}

fn solve(input: &String) {
    let mut map: Vec<Vec<bool>> = Vec::new();

    let rows: Vec<&str> = input.trim().split("\n").collect();

    let mut guard_location: (isize, isize) = (0, 0);

    for (y, row) in rows.iter().enumerate() {
        let mut bool_row: Vec<bool> = Vec::new();

        for (x, character) in row.chars().enumerate() {
            match character {
                '.' => bool_row.push(false),
                '#' => bool_row.push(true),
                '^' => {
                    bool_row.push(false);
                    guard_location = (x as isize, y as isize);
                }
                _ => {}
            }
        }

        map.push(bool_row);
    }

    if let Some(guard_path) = simulate_guard(&map, &guard_location) {
        println!("Normal path length: {}", guard_path.len());
    }

    let mut loop_position_count = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if (x == guard_location.0 as usize) & (y == guard_location.1 as usize) {
                continue;
            }

            if map[y][x] == false {
                let mut map_clone = map.clone();

                map_clone[y][x] = true;

                if simulate_guard(&map_clone, &guard_location) == None {
                    loop_position_count += 1;
                }
            }
        }
    }

    println!("Looped positions count: {}", loop_position_count);
}

fn simulate_guard(
    map: &Vec<Vec<bool>>,
    guard_start_location: &(isize, isize),
) -> Option<Vec<(isize, isize)>> {
    let mut guard_direction: (isize, isize) = (0, 1);
    let mut guard_path: Vec<(isize, isize)> = Vec::new();
    let mut guard_directions: Vec<(isize, isize)> = Vec::new();

    let mut guard_location = *guard_start_location;

    guard_path.push(guard_location);
    guard_directions.push(guard_direction);

    loop {
        let new_location = (
            guard_location.0 + guard_direction.0,
            guard_location.1 - guard_direction.1,
        );

        if (new_location.0 >= 0)
            & (new_location.0 < map[0].len() as isize)
            & (new_location.1 >= 0)
            & (new_location.1 < map.len() as isize)
        {
            if map[new_location.1 as usize][new_location.0 as usize] {
                match guard_direction {
                    (0, 1) => guard_direction = (1, 0),
                    (1, 0) => guard_direction = (0, -1),
                    (0, -1) => guard_direction = (-1, 0),
                    (-1, 0) => guard_direction = (0, 1),
                    _ => {}
                }

                continue;
            }

            guard_location = new_location;

            let mut repeat_location: bool = false;

            for i in 0..guard_path.len() {
                if guard_location == guard_path[i] {
                    repeat_location = true;

                    if guard_direction == guard_directions[i] {
                        return None;
                    }
                }
            }

            if !repeat_location {
                guard_path.push(guard_location);
                guard_directions.push(guard_direction);
            }
        } else {
            return Some(guard_path);
        }
    }
}
