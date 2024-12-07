use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("input/day02.txt").expect("Failed to read file.");

    solve(input);
}

pub fn solve(input: String) {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    let input_rows = input.split("\n");

    for row in input_rows {
        if row.len() <= 0 {
            continue;
        }

        reports.push(
            row.split_whitespace()
                .map(|n| n.parse::<i32>().expect("Couldn't parse number."))
                .collect(),
        );
    }

    let mut safe_reports: i32 = 0;
    let mut dampened_safe_reports: i32 = 0;

    for report in reports {
        if !is_safe(&report) {
            let mut is_dampened_safe: bool = false;

            for value_index in 0..report.len() {
                let mut dampened_report = report.clone();

                dampened_report.remove(value_index);

                if is_safe(&dampened_report) {
                    is_dampened_safe = true;

                    dampened_safe_reports += 1;

                    break;
                }
            }

            if !is_dampened_safe {
                continue;
            }
        } else {
            safe_reports += 1;
            dampened_safe_reports += 1;
        }
    }

    println!("Safe reports: {safe_reports}\nDampened safe reports: {dampened_safe_reports}",);
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut safe: bool = true;
    let mut value_iterator = report.iter();

    let mut previous_value: &i32 = value_iterator.next().expect("No values found in row.");

    let ascending: bool = if report[0] < report[1] { true } else { false };

    for value in value_iterator {
        let value_difference = (value - previous_value).abs();

        if (ascending && value < previous_value)
            || (!ascending && value > previous_value)
            || value_difference > 3
            || value_difference < 1
        {
            safe = false;

            break;
        }

        previous_value = value;
    }

    return safe;
}
