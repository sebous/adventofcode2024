use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    input.lines().for_each(|l| {
        let mut split = l.split_whitespace();
        left.push(split.next().unwrap().parse().unwrap());
        right.push(split.next().unwrap().parse().unwrap());
    });

    left.sort();
    right.sort();

    let result = left
        .iter()
        .zip(right)
        .fold(0, |total, (x, y)| total + (x.abs_diff(y)));

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: HashMap<u32, u32> = HashMap::new();

    input.lines().for_each(|l| {
        let mut split = l.split_whitespace();
        left.push(split.next().unwrap().parse().unwrap());
        let no = &split.next().unwrap().parse().unwrap();
        if let Some(count) = right.get_mut(no) {
            *count += 1;
        } else {
            right.insert(*no, 1);
        }
    });

    let result = left.iter().fold(0, |total, no| {
        total + no * right.get(no).or(Some(&0)).unwrap()
    });

    Some(result)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }

//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
// }
