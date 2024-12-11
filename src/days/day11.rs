use std::{collections::HashMap, fs};

pub fn run() {
    let input: String = fs::read_to_string("input/day11.txt").expect("Failed to read file.");

    solve(&input);
}

fn solve(input: &String) {
    let numbers: Vec<u64> = input
        .trim()
        .split(" ")
        .map(|num| num.parse().expect("Coudln't parse number."))
        .collect();

    let mut number_count: u64 = 0;

    let mut transposition_table: HashMap<u64, Vec<Option<u64>>> = HashMap::new();

    for number in &numbers {
        number_count += recursive_evolve(*number, 75, &mut transposition_table);
    }

    println!("Recursive: {}", number_count);
}

fn recursive_evolve(
    number: u64,
    depth: u64,
    transposition_table: &mut HashMap<u64, Vec<Option<u64>>>,
) -> u64 {
    if depth <= 0 {
        return 1;
    }

    if let Some(counts) = transposition_table.get_mut(&number) {
        if let Some(count) = counts[depth as usize] {
            return count;
        }
    } else {
        transposition_table.insert(number, vec![None; 76]);
    }

    let mut number_count: u64 = 0;

    let digits: u32 = number.checked_ilog10().unwrap_or(0) + 1;

    if number == 0 {
        number_count += recursive_evolve(1, depth - 1, transposition_table);
    } else if digits % 2 == 0 {
        let num1: u64 = number / (10u64.pow(digits / 2)) as u64;
        let num2: u64 = number - (num1 * 10u64.pow(digits / 2));

        number_count += recursive_evolve(num1, depth - 1, transposition_table);
        number_count += recursive_evolve(num2, depth - 1, transposition_table);
    } else {
        number_count += recursive_evolve(number * 2024, depth - 1, transposition_table);
    }

    if let Some(counts) = transposition_table.get_mut(&number) {
        counts[depth as usize] = Some(number_count);
    }

    return number_count;
}
