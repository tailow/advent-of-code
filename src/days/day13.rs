use std::fs;

#[derive(Default, Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

pub fn run() {
    let input: String = fs::read_to_string("input/day13.txt").expect("Failed to read file.");

    solve(&input);
}

fn solve(input: &String) {
    let configurations: Vec<&str> = input.trim().split("\n\n").collect();

    let mut total_cost: i64 = 0;

    for configuration in configurations {
        let machine: ClawMachine = parse_machine(configuration);

        if let Some(tokens) = find_minimum_tokens(machine) {
            total_cost += tokens;
        }
    }

    println!("Total cost: {}", total_cost);
}

fn parse_machine(configuration: &str) -> ClawMachine {
    let mut machine: ClawMachine = ClawMachine::default();

    let configuration_lines: Vec<&str> = configuration.split("\n").collect();

    let button_a_parts: Vec<&str> = configuration_lines[0].split(',').collect();

    let button_a_x: i64 = button_a_parts[0].split('+').collect::<Vec<&str>>()[1]
        .parse()
        .expect("Coudln't parse number.");
    let button_a_y: i64 = button_a_parts[1].split('+').collect::<Vec<&str>>()[1]
        .parse()
        .expect("Coudln't parse number.");

    let button_b_parts: Vec<&str> = configuration_lines[1].split(',').collect();

    let button_b_x: i64 = button_b_parts[0].split('+').collect::<Vec<&str>>()[1]
        .parse()
        .expect("Coudln't parse number.");
    let button_b_y: i64 = button_b_parts[1].split('+').collect::<Vec<&str>>()[1]
        .parse()
        .expect("Coudln't parse number.");

    let prize_parts: Vec<&str> = configuration_lines[2].split(',').collect();

    let prize_x: i64 = prize_parts[0].split('=').collect::<Vec<&str>>()[1]
        .parse()
        .expect("Coudln't parse number.");
    let prize_y: i64 = prize_parts[1].split('=').collect::<Vec<&str>>()[1]
        .parse()
        .expect("Coudln't parse number.");

    machine.button_a = (button_a_x, button_a_y);
    machine.button_b = (button_b_x, button_b_y);
    machine.prize = (prize_x + 10000000000000, prize_y + 10000000000000);

    return machine;
}

fn find_minimum_tokens(machine: ClawMachine) -> Option<i64> {
    let b: i64 = (machine.button_a.0 * machine.prize.1 - machine.prize.0 * machine.button_a.1)
        / (machine.button_a.0 * machine.button_b.1 - machine.button_a.1 * machine.button_b.0);

    let a: i64 = (machine.prize.0 - b * machine.button_b.0) / machine.button_a.0;

    if ((machine.button_a.0 * machine.prize.1 - machine.prize.0 * machine.button_a.1)
        % (machine.button_a.0 * machine.button_b.1 - machine.button_a.1 * machine.button_b.0)
        == 0)
        & ((machine.prize.0 - b * machine.button_b.0) % machine.button_a.0 == 0)
    {
        return Some(a * 3 + b);
    } else {
        return None;
    }
}
