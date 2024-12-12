use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Region {
    character: char,
    plots: Vec<(u32, u32)>,
    perimeter: u32,
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

    for region_type in regions {
        for region in region_type.1 {
            total_price += region.perimeter * region.plots.len() as u32;
        }
    }

    println!("Total fence price: {}", total_price);
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

            region.perimeter += 1;
        }
    } else {
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

            region.perimeter += 1;
        }
    } else {
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

            region.perimeter += 1;
        }
    } else {
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

            region.perimeter += 1;
        }
    } else {
        region.perimeter += 1;
    }
}
