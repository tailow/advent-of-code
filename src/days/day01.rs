use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("input/day01.txt").expect("Failed to read file.");

    solve(input);
}

pub fn solve(input: String) {
    let location_ids: Vec<i32> = input
        .split_whitespace()
        .map(|id| id.parse::<i32>().expect("Couldn't parse id."))
        .collect();

    let mut list1: Vec<&i32> = location_ids.iter().step_by(2).collect();
    let mut list2: Vec<&i32> = location_ids.iter().skip(1).step_by(2).collect();

    list1.sort();
    list2.sort();

    let mut total_distance: i32 = 0;
    let mut total_similarity: i32 = 0;

    for i in 0..list1.len() {
        let id1: &i32 = list1.get(i).expect("Couldn't find id.");
        let id2: &i32 = list2.get(i).expect("Coudln't find id.");

        let difference = (id1 - id2).abs();
        let occurrences: i32 = list2.iter().filter(|&id| *id == id1).count() as i32;

        total_similarity += id1 * occurrences;

        total_distance += difference;
    }

    println!("Total distance: {total_distance}");
    println!("Similarity: {total_similarity}");
}
