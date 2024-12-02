use anyhow::Result;
use itertools::Itertools;

advent_of_code::solution!(2);

fn parse(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| Ok(x.parse::<u32>()?))
                .collect()
        })
        .collect()
}

fn detect_valid(report: &Vec<u32>, skip_index: Option<usize>) -> Option<()> {
    let mut direction = 0;
    let mut valid_count = 0;

    let adjusted_report = report
        .iter()
        .enumerate()
        .filter_map(|(i, num)| {
            if skip_index.is_some_and(|skip_i| skip_i == i) {
                return None;
            }
            Some(num)
        })
        .collect_vec();

    for pair in adjusted_report.windows(2) {
        let x = pair[0];
        let y = pair[1];
        if direction == 0 {
            direction = if x > y { -1 } else { 1 };
        }

        if (x > y && direction == 1) || (x < y && direction == -1) {
            break;
        }

        if x.abs_diff(*y) < 1 || x.abs_diff(*y) > 3 {
            break;
        }

        valid_count += 1;
    }

    if adjusted_report.len() == valid_count + 1 {
        return Some(());
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input).unwrap();
    let result = input
        .iter()
        .filter_map(|report| detect_valid(report, None))
        .count();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input).unwrap();
    let count = input
        .iter()
        .filter_map(|report| {
            let valid = detect_valid(&report, None);
            if valid.is_some() {
                return valid;
            }

            (0..report.len())
                .any(|i| detect_valid(&report, Some(i)).is_some())
                .then_some(())
        })
        .count();
    Some(count as u32)
}
