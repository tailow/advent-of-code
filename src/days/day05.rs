use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("input/day05.txt").expect("Failed to read file.");

    solve(&input);
}

fn solve(input: &String) {
    let split_string: Vec<&str> = input.trim().split("\n\n").collect();

    let ordering_rules: Vec<(i32, i32)> = split_string[0]
        .split("\n")
        .map(|s| {
            let parts: Vec<&str> = s.split("|").collect();
            (
                parts[0].parse().expect("Couldn't parse rule."),
                parts[1].parse().expect("Couldn't parse rule."),
            )
        })
        .collect();

    let updates: Vec<Vec<i32>> = split_string[1]
        .split("\n")
        .map(|s| {
            s.split(",")
                .map(|n| n.parse::<i32>().expect("Coudln't parse page number."))
                .collect()
        })
        .collect();

    let mut total: i32 = 0;
    let mut ordered_total: i32 = 0;

    for update in updates {
        if is_correct(&update, &ordering_rules) {
            total += update[update.len() / 2];
        } else {
            let ordered_update: Vec<i32> = reorder(update, &ordering_rules);

            ordered_total += ordered_update[ordered_update.len() / 2];
        }
    }

    println!(
        "Middle page number total: {}\nOrdered middle page total: {}",
        total, ordered_total
    );
}

fn is_correct(pages: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for (i, page) in pages.iter().enumerate() {
        for j in 0..i {
            for rule in rules {
                if (*page == rule.0) & (pages[j] == rule.1) {
                    return false;
                }
            }
        }
    }

    return true;
}

fn reorder(pages: Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut ordered_pages = pages.clone();

    while !is_correct(&ordered_pages, rules) {
        for (i, page) in ordered_pages.clone().iter().enumerate() {
            for j in 0..i {
                for rule in rules {
                    if (*page == rule.0) & (ordered_pages[j] == rule.1) {
                        ordered_pages.swap(i, j);
                    }
                }
            }
        }
    }

    return ordered_pages;
}
