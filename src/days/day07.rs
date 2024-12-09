use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("input/day07.txt").expect("Failed to read file.");

    solve(&input);
}

fn solve(input: &String) {
    let rows: Vec<(usize, Vec<usize>)> = input
        .trim()
        .split("\n")
        .map(|row| {
            let split_row: Vec<&str> = row.split(":").collect();

            let test_value: usize = split_row[0].parse().expect("Couldn't parse test value.");
            let numbers: Vec<usize> = split_row[1]
                .trim()
                .split(" ")
                .map(|num| num.parse().expect("Coudln't parse number."))
                .collect();

            (test_value, numbers)
        })
        .collect();

    let mut total: usize = 0;

    for row in rows {
        let test_value: usize = row.0;
        let numbers: Vec<usize> = row.1;

        if recursive_operation(&numbers, test_value) {
            total += test_value;
        }
    }

    println!("Total of operations: {}", total);
}

fn recursive_operation(numbers: &Vec<usize>, test_value: usize) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == test_value;
    }

    let multiply_result: usize = numbers[0] * numbers[1];
    let addition_result: usize = numbers[0] + numbers[1];
    let concat_result: usize = numbers[0] * 10usize.pow(numbers[1].ilog10() + 1) + numbers[1];

    let mut multiply_numbers: Vec<usize> = numbers.clone();
    let mut addition_numbers: Vec<usize> = numbers.clone();
    let mut concat_numbers: Vec<usize> = numbers.clone();

    multiply_numbers.remove(0);
    multiply_numbers.remove(0);
    multiply_numbers.insert(0, multiply_result);

    addition_numbers.remove(0);
    addition_numbers.remove(0);
    addition_numbers.insert(0, addition_result);

    concat_numbers.remove(0);
    concat_numbers.remove(0);
    concat_numbers.insert(0, concat_result);

    let multiply_match: bool = recursive_operation(&multiply_numbers, test_value);
    let addition_match: bool = recursive_operation(&addition_numbers, test_value);
    let concat_match: bool = recursive_operation(&concat_numbers, test_value);

    if multiply_match || addition_match || concat_match {
        return true;
    } else {
        return false;
    }
}
