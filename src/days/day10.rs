use std::fs;

#[derive(Debug, Clone)]
struct Vector2 {
    x: i32,
    y: i32,
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

    fn multiply(self: &Vector2, other: i32) -> Vector2 {
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
    let input: String = fs::read_to_string("input/day10.txt").expect("Failed to read file.");

    solve(&input);
}

fn solve(input: &String) {
    let map: Vec<Vec<i32>> = input
        .trim()
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|ch| ch.to_digit(10).expect("Couldn't parse digit.") as i32)
                .collect()
        })
        .collect();

    let trailheads: Vec<Vector2> = find_trailheads(&map);

    let mut total_score: i32 = 0;
    let mut total_ratings: i32 = 0;

    for trailhead in &trailheads {
        let searched_locations: Vec<Vector2> = Vec::new();

        let mut reached_peaks: Vec<Vector2> = Vec::new();

        let rating =
            recursive_trail_search(&map, &trailhead, &searched_locations, &mut reached_peaks);

        total_ratings += rating;
        total_score += reached_peaks.len() as i32;
    }

    println!(
        "Total trailhead score: {}\nTotal ratings: {}",
        total_score, total_ratings
    );
}

fn find_trailheads(map: &Vec<Vec<i32>>) -> Vec<Vector2> {
    let mut trailheads: Vec<Vector2> = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                trailheads.push(Vector2 {
                    x: x as i32,
                    y: y as i32,
                })
            }
        }
    }

    return trailheads;
}

fn recursive_trail_search(
    map: &Vec<Vec<i32>>,
    start_position: &Vector2,
    searched_locations: &Vec<Vector2>,
    reached_peaks: &mut Vec<Vector2>,
) -> i32 {
    if map[start_position.y as usize][start_position.x as usize] == 9 {
        if !reached_peaks.contains(start_position) {
            reached_peaks.push(start_position.clone());
        }

        return 1;
    }

    let mut rating: i32 = 0;

    let mut searched_locations: Vec<Vector2> = searched_locations.clone();

    searched_locations.push(start_position.clone());

    let right: Vector2 = start_position.add(&Vector2 { x: 1, y: 0 });

    if in_bounds(&right, &map) {
        if (map[right.y as usize][right.x as usize]
            == map[start_position.y as usize][start_position.x as usize] + 1)
            & !searched_locations.contains(&right)
        {
            rating += recursive_trail_search(&map, &right, &searched_locations, reached_peaks);
        }
    }

    let down: Vector2 = start_position.add(&Vector2 { x: 0, y: 1 });

    if in_bounds(&down, &map) {
        if (map[down.y as usize][down.x as usize]
            == map[start_position.y as usize][start_position.x as usize] + 1)
            & !searched_locations.contains(&down)
        {
            rating += recursive_trail_search(&map, &down, &searched_locations, reached_peaks);
        }
    }

    let left: Vector2 = start_position.add(&Vector2 { x: -1, y: 0 });

    if in_bounds(&left, &map) {
        if (map[left.y as usize][left.x as usize]
            == map[start_position.y as usize][start_position.x as usize] + 1)
            & !searched_locations.contains(&left)
        {
            rating += recursive_trail_search(&map, &left, &searched_locations, reached_peaks);
        }
    }

    let up: Vector2 = start_position.add(&Vector2 { x: 0, y: -1 });

    if in_bounds(&up, &map) {
        if (map[up.y as usize][up.x as usize]
            == map[start_position.y as usize][start_position.x as usize] + 1)
            & !searched_locations.contains(&up)
        {
            rating += recursive_trail_search(&map, &up, &searched_locations, reached_peaks);
        }
    }

    return rating;
}

fn in_bounds(position: &Vector2, map: &Vec<Vec<i32>>) -> bool {
    (position.x >= 0)
        & (position.y >= 0)
        & (position.x < map[0].len() as i32)
        & (position.y < map.len() as i32)
}
