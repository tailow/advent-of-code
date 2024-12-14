use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Region {
    character: char,
    plots: Vec<(u32, u32)>,
    perimeter: u32,
    edges: Vec<Vec<(u32, u32)>>,
    up_edge_plots: Vec<(u32, u32)>,
    down_edge_plots: Vec<(u32, u32)>,
    left_edge_plots: Vec<(u32, u32)>,
    right_edge_plots: Vec<(u32, u32)>,
}

pub fn run() {
    let input: String = fs::read_to_string("input/day12.txt").expect("Failed to read file.");

    solve(&input);
}

fn solve(input: &String) {
    let plots: Vec<Vec<char>> = input
        .trim()
        .split("\n")
        .map(|row| row.chars().collect())
        .collect();

    let regions: HashMap<char, Vec<Region>> = find_regions(&plots);

    let mut total_price: u32 = 0;
    let mut total_edge_plots: u32 = 0;
    let mut total_edges: u32 = 0;

    for region_type in regions {
        for mut region in region_type.1 {
            total_edge_plots += region.down_edge_plots.len() as u32;
            total_edge_plots += region.up_edge_plots.len() as u32;
            total_edge_plots += region.left_edge_plots.len() as u32;
            total_edge_plots += region.right_edge_plots.len() as u32;

            find_edges(&mut region);

            total_edges += region.edges.len() as u32;

            total_price += region.edges.len() as u32 * region.plots.len() as u32;
        }
    }

    println!(
        "Total fence price: {}\nTotal edge plots: {}\nTotal edges: {}",
        total_price, total_edge_plots, total_edges
    );
}

fn find_regions(plots: &Vec<Vec<char>>) -> HashMap<char, Vec<Region>> {
    let mut regions: HashMap<char, Vec<Region>> = HashMap::new();

    let mut new_region_plots: Vec<(u32, u32)> = vec![(0, 0)];

    while new_region_plots.len() > 0 {
        let new_region_character =
            plots[new_region_plots[0].1 as usize][new_region_plots[0].0 as usize];

        if let Some(regions) = regions.get_mut(&new_region_character) {
            let mut region_exists: bool = false;

            for region in regions.clone() {
                if region.plots.contains(&new_region_plots[0]) {
                    region_exists = true;

                    break;
                }
            }

            if !region_exists {
                let (region, neighboring_region_plots) = find_region(new_region_plots[0], &plots);

                new_region_plots.append(&mut neighboring_region_plots.clone());

                regions.push(region);
            }
        } else {
            let (region, neighboring_region_plots) = find_region(new_region_plots[0], &plots);

            new_region_plots.append(&mut neighboring_region_plots.clone());

            regions.insert(region.character, vec![region]);
        }

        new_region_plots.remove(0);
    }

    return regions;
}

fn find_region(start: (u32, u32), plots: &Vec<Vec<char>>) -> (Region, Vec<(u32, u32)>) {
    let region_character: char = plots[start.1 as usize][start.0 as usize];

    let mut region: Region = Region {
        character: region_character,
        plots: Vec::new(),
        perimeter: 0,
        edges: Vec::new(),
        up_edge_plots: Vec::new(),
        down_edge_plots: Vec::new(),
        left_edge_plots: Vec::new(),
        right_edge_plots: Vec::new(),
    };

    let mut neighboring_region_plots: Vec<(u32, u32)> = Vec::new();

    recursive_find_region_plots(start, plots, &mut region, &mut neighboring_region_plots);

    return (region, neighboring_region_plots);
}

fn recursive_find_region_plots(
    plot: (u32, u32),
    plots: &Vec<Vec<char>>,
    region: &mut Region,
    neighboring_region_plots: &mut Vec<(u32, u32)>,
) {
    region.plots.push(plot);

    // Left
    if plot.0 > 0 {
        let left = (plot.0 - 1, plot.1);

        if plots[left.1 as usize][(left.0) as usize] == region.character {
            if !region.plots.contains(&left) {
                recursive_find_region_plots(left, plots, region, neighboring_region_plots);
            }
        } else {
            neighboring_region_plots.push(left);

            region.left_edge_plots.push(plot);

            region.perimeter += 1;
        }
    } else {
        region.left_edge_plots.push(plot);

        region.perimeter += 1;
    }

    // Right
    if plot.0 < (plots[0].len() - 1) as u32 {
        let right = (plot.0 + 1, plot.1);

        if plots[right.1 as usize][(right.0) as usize] == region.character {
            if !region.plots.contains(&right) {
                recursive_find_region_plots(right, plots, region, neighboring_region_plots);
            }
        } else {
            neighboring_region_plots.push(right);

            region.right_edge_plots.push(plot);

            region.perimeter += 1;
        }
    } else {
        region.right_edge_plots.push(plot);

        region.perimeter += 1;
    }

    // Up
    if plot.1 > 0 {
        let up = (plot.0, plot.1 - 1);

        if plots[up.1 as usize][(up.0) as usize] == region.character {
            if !region.plots.contains(&up) {
                recursive_find_region_plots(up, plots, region, neighboring_region_plots);
            }
        } else {
            neighboring_region_plots.push(up);

            region.up_edge_plots.push(plot);

            region.perimeter += 1;
        }
    } else {
        region.up_edge_plots.push(plot);

        region.perimeter += 1;
    }

    // Down
    if plot.1 < (plots.len() - 1) as u32 {
        let down = (plot.0, plot.1 + 1);

        if plots[down.1 as usize][(down.0) as usize] == region.character {
            if !region.plots.contains(&down) {
                recursive_find_region_plots(down, plots, region, neighboring_region_plots);
            }
        } else {
            neighboring_region_plots.push(down);

            region.down_edge_plots.push(plot);

            region.perimeter += 1;
        }
    } else {
        region.down_edge_plots.push(plot);

        region.perimeter += 1;
    }
}

fn find_edges(region: &mut Region) {
    while region.up_edge_plots.len() > 0 {
        let mut current_edge: Vec<(u32, u32)> = Vec::new();

        current_edge.push(region.up_edge_plots[0]);

        region.up_edge_plots.remove(0);

        let mut index = 0;

        while index < region.up_edge_plots.len() {
            let mut is_in_edge: bool = false;

            for edge in &current_edge {
                if (region.up_edge_plots[index].1 == edge.1)
                    & ((region.up_edge_plots[index].0 == edge.0 - 1)
                        || (region.up_edge_plots[index].0 == edge.0 + 1))
                {
                    is_in_edge = true;

                    break;
                }
            }

            if is_in_edge {
                current_edge.push(region.up_edge_plots.remove(index));

                index = 0;
            } else {
                index += 1;
            }
        }

        region.edges.push(current_edge);
    }

    while region.down_edge_plots.len() > 0 {
        let mut current_edge: Vec<(u32, u32)> = Vec::new();

        current_edge.push(region.down_edge_plots[0]);

        region.down_edge_plots.remove(0);

        let mut index = 0;

        while index < region.down_edge_plots.len() {
            let mut is_in_edge: bool = false;

            for edge in &current_edge {
                if (region.down_edge_plots[index].1 == edge.1)
                    & ((region.down_edge_plots[index].0 == edge.0 - 1)
                        || (region.down_edge_plots[index].0 == edge.0 + 1))
                {
                    is_in_edge = true;

                    break;
                }
            }

            if is_in_edge {
                current_edge.push(region.down_edge_plots.remove(index));

                index = 0;
            } else {
                index += 1;
            }
        }

        region.edges.push(current_edge);
    }

    while region.right_edge_plots.len() > 0 {
        let mut current_edge: Vec<(u32, u32)> = Vec::new();

        current_edge.push(region.right_edge_plots[0]);

        region.right_edge_plots.remove(0);

        let mut index = 0;

        while index < region.right_edge_plots.len() {
            let mut is_in_edge: bool = false;

            for edge in &current_edge {
                if (region.right_edge_plots[index].0 == edge.0)
                    & ((region.right_edge_plots[index].1 == edge.1 - 1)
                        || (region.right_edge_plots[index].1 == edge.1 + 1))
                {
                    is_in_edge = true;

                    break;
                }
            }

            if is_in_edge {
                current_edge.push(region.right_edge_plots.remove(index));

                index = 0;
            } else {
                index += 1;
            }
        }

        region.edges.push(current_edge);
    }

    while region.left_edge_plots.len() > 0 {
        let mut current_edge: Vec<(u32, u32)> = Vec::new();

        current_edge.push(region.left_edge_plots[0]);

        region.left_edge_plots.remove(0);

        let mut index = 0;

        while index < region.left_edge_plots.len() {
            let mut is_in_edge: bool = false;

            for edge in &current_edge {
                if (region.left_edge_plots[index].0 == edge.0)
                    & ((region.left_edge_plots[index].1 == edge.1 - 1)
                        || (region.left_edge_plots[index].1 == edge.1 + 1))
                {
                    is_in_edge = true;

                    break;
                }
            }

            if is_in_edge {
                current_edge.push(region.left_edge_plots.remove(index));

                index = 0;
            } else {
                index += 1;
            }
        }

        region.edges.push(current_edge);
    }
}
