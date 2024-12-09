use std::collections::{HashMap, HashSet};

use advent_of_code::grid::{Coord, Grid};
use itertools::Itertools;

advent_of_code::solution!(8);

fn parse(input: &str) -> Grid<char> {
    let mut map: HashMap<_, _> = HashMap::new();
    let height = input.lines().count();
    let mut width = 0;
    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            width = line.len();
        }
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => continue,
                _ => map.insert((x, y), ch),
            };
        }
    }

    Grid { height, width, map }
}

fn add_diff(a: &usize, b: &usize, diff: usize) -> Option<usize> {
    if a < b {
        a.checked_sub(diff)
    } else {
        a.checked_add(diff)
    }
}

fn calc_antinodes(
    (ax, ay): &Coord,
    (bx, by): &Coord,
    grid: &Grid<char>,
    resonance: bool,
) -> Vec<Coord> {
    let diff_x = ax.abs_diff(*bx);
    let diff_y = ay.abs_diff(*by);

    let mut nodes = vec![];

    if !resonance {
        let x1 = add_diff(ax, bx, diff_x);
        let y1 = add_diff(ay, by, diff_y);
        let x2 = add_diff(bx, ax, diff_x);
        let y2 = add_diff(by, ay, diff_y);

        if x1.is_some_and(|x| x < grid.width) && y1.is_some_and(|y| y < grid.height) {
            nodes.push((x1.unwrap(), y1.unwrap()));
        }

        if x2.is_some_and(|x| x < grid.width) && y2.is_some_and(|y| y < grid.height) {
            nodes.push((x2.unwrap(), y2.unwrap()));
        }
        return nodes;
    }

    nodes.push((*ax, *ay));
    nodes.push((*bx, *by));
    let mut multi = 1;
    loop {
        let x = add_diff(ax, bx, diff_x * multi);
        let y = add_diff(ay, by, diff_y * multi);

        if x.is_some_and(|x| x < grid.width) && y.is_some_and(|y| y < grid.height) {
            nodes.push((x.unwrap(), y.unwrap()));
            multi += 1;
        } else {
            break;
        }
    }
    multi = 1;

    loop {
        let x = add_diff(bx, ax, diff_x * multi);
        let y = add_diff(by, ay, diff_y * multi);

        if x.is_some_and(|x| x < grid.width) && y.is_some_and(|y| y < grid.height) {
            nodes.push((x.unwrap(), y.unwrap()));
            multi += 1;
        } else {
            break;
        }
    }

    nodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let frequencies = grid.map.values().unique().collect_vec();
    let mut antinodes = HashSet::new();

    for freq in frequencies {
        for pair in grid
            .map
            .iter()
            .filter_map(|(coord, ch)| ch.eq(freq).then_some(coord))
            .combinations(2)
        {
            calc_antinodes(pair[0], pair[1], &grid, false)
                .iter()
                .for_each(|n| {
                    antinodes.insert(n.clone());
                });
        }
    }
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    let frequencies = grid.map.values().unique().collect_vec();
    let mut antinodes = HashSet::new();

    for freq in frequencies {
        for pair in grid
            .map
            .iter()
            .filter_map(|(coord, ch)| ch.eq(freq).then_some(coord))
            .combinations(2)
        {
            calc_antinodes(pair[0], pair[1], &grid, true)
                .iter()
                .for_each(|n| {
                    antinodes.insert(n.clone());
                });
        }
    }
    Some(antinodes.len() as u32)
}
