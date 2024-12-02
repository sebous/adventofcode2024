use itertools::Itertools;

advent_of_code::solution!(2);

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

    let report_len = adjusted_report.len();

    for win in adjusted_report.windows(2) {
        let x = win[0];
        let y = win[1];
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

    if report_len == valid_count + 1 {
        return Some(());
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter_map(|l| {
            let report = l
                .split_whitespace()
                .map(|str| str.parse::<u32>().unwrap())
                .collect_vec();

            detect_valid(&report, None)
        })
        .count() as u32;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter_map(|l| {
            let report = l
                .split_whitespace()
                .map(|str| str.parse::<u32>().unwrap())
                .collect_vec();

            let valid = detect_valid(&report, None);
            if valid.is_some() {
                return valid;
            }

            (0..report.len())
                .any(|i| detect_valid(&report, Some(i)).is_some())
                .then_some(())
        })
        .count() as u32;
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
