use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Block {
    start: usize,
    size: usize,
}

#[derive(Clone, Debug)]
struct Drive {
    files: HashMap<usize, Block>,
    gaps: Vec<Block>,
    max_file_id: usize,
}

impl Drive {
    fn calculate_checksum(&self) -> u64 {
        self.files
            .iter()
            .map(|(id, block)| {
                (block.start..block.start + block.size)
                    .map(|i| (i * id) as u64)
                    .sum::<u64>()
            })
            .sum()
    }
}

fn read_disk(input: &str) -> Drive {
    let mut drive = Drive {
        files: HashMap::new(),
        gaps: Vec::new(),
        max_file_id: 0,
    };
    let mut position = 0;

    input.chars().enumerate().for_each(|(i, c)| {
        if let Some(length) = c.to_digit(10) {
            if length > 0 {
                let block = Block {
                    start: position,
                    size: length as usize,
                };
                let is_file = i % 2 == 0;

                if is_file {
                    drive.files.insert(i / 2, block);
                } else {
                    drive.gaps.push(block);
                }

                position += length as usize;
            }
        };
    });

    drive.max_file_id = drive.files.iter().map(|(id, _)| id).max().unwrap().clone();

    drive
}

fn swap_block(drive: &mut Drive, file_id: usize) -> Option<()> {
    let file = drive.files.get_mut(&file_id)?;
    let (gap_id, gap) = drive
        .gaps
        .iter()
        .enumerate()
        .skip_while(|(_, gap)| gap.size < file.size)
        .next()?
        .clone();

    if gap.start >= file.start {
        return None;
    }

    file.start = gap.start;

    if file.size == gap.size {
        drive.gaps.remove(gap_id);
    } else {
        drive.gaps[gap_id].start += file.size;
        drive.gaps[gap_id].size -= file.size;
    }

    Some(())
}

fn compress_files(drive: &mut Drive) {
    for id in (1..=drive.max_file_id).rev() {
        swap_block(drive, id);
    }
}

pub fn solve(input: &str) -> String {
    let mut disk = read_disk(&input);
    compress_files(&mut disk);
    let checksum = disk.calculate_checksum();
    format!("The checksum is: {}", checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "The checksum is: 2858";
        assert_eq!(actual_solution, expected_solution);
    }
}
