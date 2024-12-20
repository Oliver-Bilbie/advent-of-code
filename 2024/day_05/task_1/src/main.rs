use std::collections::HashMap;
use std::fs;

fn read_pages(input_file: &str) -> HashMap<u8, Vec<u8>> {
    let mut pages: HashMap<u8, Vec<u8>> = HashMap::new();

    fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            let page_num: u8 = parts[1].parse().unwrap();
            let required_page: u8 = parts[0].parse().unwrap();

            pages
                .entry(page_num)
                .and_modify(|required_pages| required_pages.push(required_page))
                .or_insert(vec![required_page]);
        });

    pages
}

fn read_reports(input_file: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn first_invalid_report_page(report: &Vec<u8>, pages: &HashMap<u8, Vec<u8>>) -> Option<u8> {
    for i in 0..report.len() {
        let page = report[i];
        let remaining_pages = &report[i + 1..];
        let required_pages = pages.get(&page).unwrap_or(&vec![]).clone();

        for required_page in required_pages {
            if remaining_pages.contains(&required_page) {
                return Some(i as u8);
            }
        }
    }

    None
}

fn remove_invalid_reports(reports: &Vec<Vec<u8>>, pages: &HashMap<u8, Vec<u8>>) -> Vec<Vec<u8>> {
    reports
        .iter()
        .filter(|report| first_invalid_report_page(&report, &pages).is_none())
        .cloned()
        .collect()
}

fn find_middle_page(report: &Vec<u8>) -> u8 {
    // Since the input only contains reports with an odd number of pages, we can keep this simple
    report[(report.len() - 1) / 2]
}

fn main() {
    let pages = read_pages("../input.txt");
    let mut reports = read_reports("../input.txt");

    reports = remove_invalid_reports(&mut reports, &pages);

    let sum_middle_pages: u64 = reports
        .iter()
        .map(|report| find_middle_page(report) as u64)
        .sum();

    println!(
        "The sum of the middle page numbers of valid reports is {}",
        sum_middle_pages
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_pages() {
        let actual_pages = read_pages("../test_input.txt");
        let expected_pages: HashMap<u8, Vec<u8>> = HashMap::from([
            (53, vec![47, 75, 61, 97]),
            (13, vec![97, 61, 29, 47, 75, 53]),
            (61, vec![97, 47, 75]),
            (29, vec![75, 97, 53, 61, 47]),
            (47, vec![97, 75]),
            (75, vec![97]),
        ]);
        assert_eq!(actual_pages, expected_pages);
    }

    #[test]
    fn it_reads_the_reports() {
        let actual_reports = read_reports("../test_input.txt");
        let expected_reports = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        assert_eq!(actual_reports, expected_reports);
    }

    #[test]
    fn it_solves_line_1() {
        let pages = read_pages("../test_input.txt");
        let report = vec![75, 47, 61, 53, 29];
        let is_valid = first_invalid_report_page(&report, &pages).is_none();
        assert!(is_valid);
    }

    #[test]
    fn it_solves_line_2() {
        let pages = read_pages("../test_input.txt");
        let report = vec![97, 61, 53, 29, 13];
        let is_valid = first_invalid_report_page(&report, &pages).is_none();
        assert!(is_valid);
    }

    #[test]
    fn it_solves_line_3() {
        let pages = read_pages("../test_input.txt");
        let report = vec![75, 29, 13];
        let is_valid = first_invalid_report_page(&report, &pages).is_none();
        assert!(is_valid);
    }

    #[test]
    fn it_solves_line_4() {
        let pages = read_pages("../test_input.txt");
        let report = vec![75, 97, 47, 61, 53];
        let is_valid = first_invalid_report_page(&report, &pages).is_none();
        assert!(!is_valid);
    }

    #[test]
    fn it_solves_line_5() {
        let pages = read_pages("../test_input.txt");
        let report = vec![61, 13, 29];
        let is_valid = first_invalid_report_page(&report, &pages).is_none();
        assert!(!is_valid);
    }

    #[test]
    fn it_solves_line_6() {
        let pages = read_pages("../test_input.txt");
        let report = vec![97, 13, 75, 29, 47];
        let is_valid = first_invalid_report_page(&report, &pages).is_none();
        assert!(!is_valid);
    }

    #[test]
    fn it_solves_the_example() {
        let pages = read_pages("../test_input.txt");
        let mut reports = read_reports("../test_input.txt");
        reports = remove_invalid_reports(&mut reports, &pages);
        let actual_sum: u64 = reports
            .iter()
            .map(|report| find_middle_page(report) as u64)
            .sum();
        let expected_sum = 143;
        assert_eq!(actual_sum, expected_sum);
    }
}
