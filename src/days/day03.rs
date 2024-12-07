use regex::Regex;
use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("input/day03.txt").expect("Failed to read file.");

    solve(input);
}

fn solve(input: String) {
    let mul_regex = Regex::new(r"^mul\((?<f>[0-9]+),(?<s>[0-9]+)\)").unwrap();
    let do_regex = Regex::new(r"^do\(\)").unwrap();
    let dont_regex = Regex::new(r"^don't\(\)").unwrap();

    let mut multiply: bool = true;

    let mut total: i32 = 0;

    for (i, _) in input.chars().enumerate() {
        if let Some(capture) = mul_regex.captures(&input[i..]) {
            let first_value: i32 = capture
                .name("f")
                .unwrap()
                .as_str()
                .parse()
                .expect("Couldn't parse value.");

            let second_value: i32 = capture
                .name("s")
                .unwrap()
                .as_str()
                .parse()
                .expect("Couldn't parse value.");

            if multiply {
                total += first_value * second_value;
            }
        } else if do_regex.is_match(&input[i..]) {
            multiply = true;
        } else if dont_regex.is_match(&input[i..]) {
            multiply = false;
        }
    }

    println!("Multiplication total: {total}");
}
