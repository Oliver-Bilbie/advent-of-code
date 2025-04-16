fn read_disk(input: &str) -> Vec<Option<usize>> {
    let mut disk: Vec<Option<usize>> = vec![];

    input.chars().enumerate().for_each(|(i, c)| {
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

fn swap_pair(disk: &mut [Option<usize>]) -> bool {
    let first_space_index = disk
        .iter()
        .enumerate()
        .find_map(|(i, item)| if item.is_none() { Some(i) } else { None })
        .unwrap();
    let last_item_index = disk
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, item)| if item.is_some() { Some(i) } else { None })
        .unwrap();

    if first_space_index > last_item_index {
        return false;
    } else {
        disk[first_space_index] = disk[last_item_index];
        disk[last_item_index] = None;
        return true;
    }
}

fn compress_files(disk: &mut [Option<usize>]) {
    let mut swap_successful = swap_pair(disk);
    while swap_successful {
        swap_successful = swap_pair(disk);
    }
}

pub fn solve(input: &str) -> String {
    let mut disk = read_disk(&input);
    compress_files(&mut disk);
    let checksum = calculate_checksum(&disk);
    format!("The checksum is: {}", checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_example_disk() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_disk = read_disk(&input);
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
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "The checksum is: 1928";
        assert_eq!(actual_solution, expected_solution);
    }
}
