use std::fs;

#[derive(Default, Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn move_robot(self: &mut Robot, map_width: &i32, map_height: &i32) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        if self.position.0 >= *map_width {
            self.position.0 = self.position.0 % map_width;
        } else if self.position.0 < 0 {
            self.position.0 += map_width;
        }

        if self.position.1 >= *map_height {
            self.position.1 = self.position.1 % map_height;
        } else if self.position.1 < 0 {
            self.position.1 += map_height;
        }
    }
}

pub fn run() {
    let input: String = fs::read_to_string("input/day14.txt").expect("Failed to read file.");

    solve(&input);
}

fn solve(input: &String) {
    let mut robots = parse_robots(input);

    let map_width: i32 = 101;
    let map_height: i32 = 103;

    let mut robot_count_top_left: i32 = 0;
    let mut robot_count_top_right: i32 = 0;
    let mut robot_count_bottom_left: i32 = 0;
    let mut robot_count_bottom_right: i32 = 0;

    for _ in 0..100 {
        for robot in &mut robots {
            robot.move_robot(&map_width, &map_height);
        }
    }

    for robot in robots {
        if (robot.position.0 < (map_width - 1) / 2) & (robot.position.1 < (map_height - 1) / 2) {
            robot_count_top_left += 1;
        } else if (robot.position.0 > (map_width - 1) / 2)
            & (robot.position.1 < (map_height - 1) / 2)
        {
            robot_count_top_right += 1;
        } else if (robot.position.0 < (map_width - 1) / 2)
            & (robot.position.1 > (map_height - 1) / 2)
        {
            robot_count_bottom_left += 1;
        } else if (robot.position.0 > (map_width - 1) / 2)
            & (robot.position.1 > (map_height - 1) / 2)
        {
            robot_count_bottom_right += 1;
        }
    }

    let safety_factor: i32 = robot_count_top_left
        * robot_count_top_right
        * robot_count_bottom_left
        * robot_count_bottom_right;

    println!("Safety factor: {}", safety_factor);

    let mut robots = parse_robots(input);

    let mut index = 0;

    while index < 10000 {
        let mut bathroom: Vec<Vec<bool>> =
            vec![vec![false; map_width as usize]; map_height as usize];

        for robot in &mut robots {
            robot.move_robot(&map_width, &map_height);

            bathroom[robot.position.1 as usize][robot.position.0 as usize] = true;
        }

        let mut consecutive_robots: i32 = 0;

        for y in 0..bathroom.len() {
            for x in 1..bathroom[y].len() {
                if bathroom[y][x] & bathroom[y][x - 1] {
                    consecutive_robots += 1;
                }
            }
        }

        index += 1;

        if consecutive_robots > 200 {
            println!("{index} {consecutive_robots}");
            print_bathroom(bathroom);
        }
    }
}

fn print_bathroom(bathroom: Vec<Vec<bool>>) {
    let mut layout: String = String::new();

    for y in 0..bathroom.len() {
        for x in 0..bathroom[y].len() {
            if bathroom[y][x] {
                layout.push('*');
            } else {
                layout.push(' ');
            }
        }

        layout.push('\n');
    }

    println!("{}", layout);
}

fn parse_robots(input: &String) -> Vec<Robot> {
    let robots: Vec<Robot> = input
        .trim()
        .split("\n")
        .map(|line| {
            let pos_string: &str = line.split(" ").collect::<Vec<&str>>()[0];
            let vel_string: &str = line.split(" ").collect::<Vec<&str>>()[1];

            let pos_values_string: &str = pos_string.split("=").collect::<Vec<&str>>()[1];
            let vel_values_string: &str = vel_string.split("=").collect::<Vec<&str>>()[1];

            let pos_x: i32 = pos_values_string.split(",").collect::<Vec<&str>>()[0]
                .parse()
                .expect("Couldn't parse int.");
            let pos_y: i32 = pos_values_string.split(",").collect::<Vec<&str>>()[1]
                .parse()
                .expect("Couldn't parse int.");

            let vel_x: i32 = vel_values_string.split(",").collect::<Vec<&str>>()[0]
                .parse()
                .expect("Couldn't parse int.");
            let vel_y: i32 = vel_values_string.split(",").collect::<Vec<&str>>()[1]
                .parse()
                .expect("Couldn't parse int.");

            return Robot {
                position: (pos_x, pos_y),
                velocity: (vel_x, vel_y),
            };
        })
        .collect();

    return robots;
}
