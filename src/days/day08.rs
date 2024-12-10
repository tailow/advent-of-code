use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Vector2 {
    x: isize,
    y: isize,
}

impl Vector2 {
    fn add(self: &Vector2, other: &Vector2) -> Vector2 {
        return Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }

    fn subtract(self: &Vector2, other: &Vector2) -> Vector2 {
        return Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }

    fn multiply(self: &Vector2, other: isize) -> Vector2 {
        return Vector2 {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) & (self.y == other.y)
    }
}

pub fn run() {
    let input: String = fs::read_to_string("input/day08.txt").expect("Failed to read file.");

    solve(&input);
}

fn solve(input: &String) {
    let mut antenna_groups: HashMap<char, Vec<Vector2>> = HashMap::new();

    let rows: Vec<&str> = input.trim().split("\n").collect();

    for (y, row) in rows.iter().enumerate() {
        for (x, character) in row.chars().enumerate() {
            if character != '.' {
                if let Some(antenna) = antenna_groups.get_mut(&character) {
                    antenna.push(Vector2 {
                        x: x as isize,
                        y: y as isize,
                    });
                } else {
                    antenna_groups.insert(
                        character,
                        vec![Vector2 {
                            x: x as isize,
                            y: y as isize,
                        }],
                    );
                }
            }
        }
    }

    let mut antinodes: Vec<Vector2> = Vec::new();

    for antenna_group in antenna_groups {
        for i in 0..antenna_group.1.len() - 1 {
            for j in i + 1..antenna_group.1.len() {
                let antenna_pair_difference: Vector2 =
                    antenna_group.1[i].subtract(&antenna_group.1[j]);

                let mut resonance_index = 1;

                loop {
                    let antinode =
                        antenna_group.1[i].add(&antenna_pair_difference.multiply(resonance_index));

                    if in_bounds(&antinode, &rows) {
                        if !antinodes.contains(&antinode) {
                            antinodes.push(antinode);
                        }
                    } else {
                        break;
                    }

                    resonance_index += 1;
                }

                resonance_index = 1;

                loop {
                    let antinode = antenna_group.1[i]
                        .subtract(&antenna_pair_difference.multiply(resonance_index));

                    if in_bounds(&antinode, &rows) {
                        if !antinodes.contains(&antinode) {
                            antinodes.push(antinode);
                        }
                    } else {
                        break;
                    }

                    resonance_index += 1;
                }

                resonance_index = 1;

                loop {
                    let antinode =
                        antenna_group.1[j].add(&antenna_pair_difference.multiply(resonance_index));

                    if in_bounds(&antinode, &rows) {
                        if !antinodes.contains(&antinode) {
                            antinodes.push(antinode);
                        }
                    } else {
                        break;
                    }

                    resonance_index += 1;
                }

                resonance_index = 1;

                loop {
                    let antinode = antenna_group.1[j]
                        .subtract(&antenna_pair_difference.multiply(resonance_index));

                    if in_bounds(&antinode, &rows) {
                        if !antinodes.contains(&antinode) {
                            antinodes.push(antinode);
                        }
                    } else {
                        break;
                    }

                    resonance_index += 1;
                }
            }
        }
    }

    println!("Antinodes count: {}", antinodes.len());
}

fn in_bounds(position: &Vector2, rows: &Vec<&str>) -> bool {
    (position.x >= 0)
        & (position.y >= 0)
        & (position.x < rows[0].len() as isize)
        & (position.y < rows.len() as isize)
}
