use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("input/day09.txt").expect("Failed to read file.");

    solve(&input);
}

fn solve(input: &String) {
    let input: &str = input.trim();

    let mut expanded_file: Vec<Option<u64>> = Vec::new();

    let mut empty_space: bool = false;

    let mut block_id = 0;

    for character in input.chars() {
        let value: u32 = character.to_digit(10).expect("Couldn't convert to digit.");

        for _ in 0..value {
            if empty_space {
                expanded_file.push(None);
            } else {
                expanded_file.push(Some(block_id));
            }
        }

        if !empty_space {
            block_id += 1;
        }

        empty_space = !empty_space;
    }

    let reordered_file = reorder_blocks(&expanded_file);
    let reordered_file_part2 = reorder_files(&expanded_file);

    let mut checksum: u64 = 0;
    let mut checksum_part2: u64 = 0;

    for (i, block) in reordered_file.iter().enumerate() {
        if let Some(block) = block {
            checksum += block * i as u64;
        }
    }

    for (i, block) in reordered_file_part2.iter().enumerate() {
        if let Some(block) = block {
            checksum_part2 += block * i as u64;
        }
    }

    println!(
        "Checksum: {}\nChecksum part 2: {}",
        checksum, checksum_part2
    );
}

fn reorder_blocks(expanded_file: &Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut reordered_file = expanded_file.clone();

    for i in (0..reordered_file.len()).rev() {
        if let Some(_) = reordered_file[i] {
            for j in 0..i {
                if reordered_file[j] == None {
                    reordered_file.swap(i, j);

                    break;
                }
            }
        }
    }

    return reordered_file;
}

fn reorder_files(expanded_file: &Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut reordered_file = expanded_file.clone();

    let mut current_block_id: u64 = 0;
    let mut current_block_length: usize = 0;

    for i in (0..reordered_file.len()).rev() {
        if let Some(block_id) = reordered_file[i] {
            if block_id == current_block_id {
                current_block_length += 1;
            } else {
                // Swap previous file
                if (current_block_id != 0) & (current_block_length > 0) {
                    reorder_file(&mut reordered_file, i + 1, current_block_length);
                }

                // Start new file
                current_block_id = block_id;
                current_block_length = 1;
            }
        } else {
            if current_block_length != 0 {
                reorder_file(&mut reordered_file, i + 1, current_block_length);

                current_block_length = 0;
                current_block_id = 0;
            }
        }
    }

    return reordered_file;
}

fn reorder_file(reordered_file: &mut Vec<Option<u64>>, file_index: usize, file_length: usize) {
    if let Some(empty_slot_index) = find_empty_slot(&reordered_file, file_length, file_index) {
        for i in 0..file_length {
            reordered_file.swap(file_index + i, empty_slot_index + i);
        }
    }
}

fn find_empty_slot(
    expanded_file: &Vec<Option<u64>>,
    slot_size: usize,
    final_index: usize,
) -> Option<usize> {
    let mut empty_slot_index: Option<usize> = None;

    let mut current_slot_size = 0;

    for i in 0..final_index {
        if expanded_file[i] == None {
            current_slot_size += 1;

            if current_slot_size >= slot_size {
                empty_slot_index = Some(i - slot_size + 1);

                break;
            }
        } else {
            current_slot_size = 0;
        }
    }

    return empty_slot_index;
}
