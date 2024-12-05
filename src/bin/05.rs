use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Data {
    rules: HashMap<u32, Vec<u32>>,
    page_sets: Vec<Vec<u32>>,
}

fn parse(input: &str) -> Data {
    let split = input.split("\n\n").collect_vec();
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    split[0].lines().for_each(|l| {
        let split = l
            .split("|")
            .map(|n| n.parse::<u32>().unwrap())
            .collect_vec();
        if let Some(r) = rules.get_mut(&split[0]) {
            r.push(split[1]);
        } else {
            rules.insert(split[0], vec![split[1]]);
        }
    });

    let page_sets = split[1]
        .lines()
        .map(|l| l.split(",").map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();
    Data { rules, page_sets }
}

fn get_valid_updates(data: &Data) -> (Vec<usize>, u32) {
    let mut total = 0;
    let mut valid_indexes = vec![];

    'outer: for (set_i, set) in data.page_sets.iter().enumerate() {
        for (i, page) in set.iter().enumerate() {
            let rules = data.rules.get(page);
            if rules.is_none() {
                continue;
            }

            for rule in rules.unwrap().iter() {
                if set.iter().enumerate().any(|(ix, x)| x == rule && ix < i) {
                    continue 'outer;
                }
            }
        }

        total += set[(set.len() - 1) / 2];
        valid_indexes.push(set_i);
    }

    (valid_indexes, total)
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);
    let (_, total) = get_valid_updates(&data);
    Some(total)
}

fn get_middle_of_sorted(data: &Data, pages_index: usize) -> u32 {
    let mut changes = 0;

    let mut current_sort = data.page_sets[pages_index].clone();
    let mut next_sort = data.page_sets[pages_index].clone();

    loop {
        for (i, page) in current_sort.iter().enumerate() {
            let rules = data.rules.get(page);
            if rules.is_none() || i == 0 {
                continue;
            }

            if let Some(_) = rules.unwrap().iter().find(|r| **r == current_sort[i - 1]) {
                next_sort.swap(i - 1, i);
                changes += 1;
            }
        }

        if changes == 0 {
            return next_sort[(next_sort.len() - 1) / 2];
        }

        current_sort = next_sort.clone();
        changes = 0;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);
    let (valid_indexes, _) = get_valid_updates(&data);

    let res = data
        .page_sets
        .iter()
        .enumerate()
        .filter(|(i, _)| !valid_indexes.contains(i))
        .fold(0, |total, (i, _)| total + get_middle_of_sorted(&data, i));

    Some(res)
}
