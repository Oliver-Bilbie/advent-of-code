use std::fs;

#[derive(Clone, Debug)]
struct Block {
    start: usize,
    size: usize,
}

fn read_disk(input_file: &str) -> Vec<Option<usize>> {
    let mut disk: Vec<Option<usize>> = vec![];

    fs::read_to_string(input_file)
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            if let Some(length) = c.to_digit(10) {
                let value = if i % 2 == 0 { Some(i / 2) } else { None };
                disk.append(&mut vec![value; length as usize]);
            };
        });

    disk
}

fn calculate_checksum(disk: &[Option<usize>]) -> u64 {
    disk.iter()
        .enumerate()
        .fold(0, |sum, (i, content)| match content {
            Some(value) => sum + (*value * i) as u64,
            None => sum,
        })
}

fn find_first_empty_block(disk: &[Option<usize>], start_i: usize) -> Option<Block> {
    let start =
        disk[start_i..]
            .iter()
            .enumerate()
            .find_map(|(i, item)| if item.is_none() { Some(i) } else { None })?
            + start_i;
    let size =
        disk[start..]
            .iter()
            .enumerate()
            .find_map(|(i, item)| if item.is_some() { Some(i) } else { None })?;

    Some(Block { start, size })
}

fn find_file_block(disk: &[Option<usize>], id: usize) -> Option<Block> {
    let start = disk.iter().enumerate().find_map(|(i, item)| match item {
        Some(value) => {
            if *value == id {
                Some(i)
            } else {
                None
            }
        }
        None => None,
    })?;
    let end = disk
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, item)| match item {
            Some(value) => {
                if *value == id {
                    Some(i)
                } else {
                    None
                }
            }
            None => None,
        })?;
    let size = end - start + 1;

    Some(Block { start, size })
}

fn swap_block(disk: &mut [Option<usize>], id: usize) -> bool {
    let file_block = find_file_block(&disk, id).unwrap();
    let mut empty_block = find_first_empty_block(&disk, 0).unwrap();

    loop {
        if empty_block.start > file_block.start {
            return false;
        }

        if empty_block.size >= file_block.size {
            for i in 0..file_block.size {
                disk[empty_block.start + i] = disk[file_block.start + i];
                disk[file_block.start + i] = None;
            }
            return true;
        }

        empty_block = match find_first_empty_block(&disk, empty_block.start + empty_block.size) {
            Some(value) => value,
            None => return false,
        };
    }
}

fn compress_files(disk: &mut [Option<usize>]) {
    let max_id = disk.iter().max().unwrap().unwrap();
    let mut state_changed: bool;

    loop {
        state_changed = false;

        for id in (0..=max_id).rev() {
            if swap_block(disk, id) {
                state_changed = true;
            }
        }

        if !state_changed {
            return;
        }

        return;
    }
}

fn main() {
    let mut disk = read_disk("../input.txt");
    compress_files(&mut disk);
    let checksum = calculate_checksum(&disk);

    println!("The checksum is: {}", checksum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_example_disk() {
        let actual_disk = read_disk("../test_input.txt");
        let expected_disk = vec![
            Some(0),
            Some(0),
            None,
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            Some(2),
            None,
            None,
            None,
            Some(3),
            Some(3),
            Some(3),
            None,
            Some(4),
            Some(4),
            None,
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(6),
            Some(6),
            Some(6),
            Some(6),
            None,
            Some(7),
            Some(7),
            Some(7),
            None,
            Some(8),
            Some(8),
            Some(8),
            Some(8),
            Some(9),
            Some(9),
        ];
        assert_eq!(actual_disk, expected_disk);
    }

    #[test]
    fn it_solves_the_example() {
        let mut disk = read_disk("../test_input.txt");
        compress_files(&mut disk);
        let actual_checksum = calculate_checksum(&disk);

        let expected_checksum = 2858;

        assert_eq!(actual_checksum, expected_checksum);
    }
}
