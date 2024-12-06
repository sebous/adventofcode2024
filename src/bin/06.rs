use std::collections::{HashMap, HashSet};

use advent_of_code::grid::{Coord, Grid};

advent_of_code::solution!(6);

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse(input: &str) -> (Grid<char>, Coord) {
    let mut start: (usize, usize) = (0, 0);
    let height = input.lines().count();
    let mut width = 0;
    let mut map: HashMap<Coord, char> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            map.insert((x, y), ch);
            if ch == '^' {
                start = (x, y);
            }
        }
        if width == 0 {
            width = line.len();
        }
    }

    (Grid { map, height, width }, start)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start) = parse(input);

    let (mut x, mut y) = start;
    let mut dir_index = 0;
    let mut visited = HashSet::from([(x, y)]);

    loop {
        let x1 = x.checked_add_signed(DIRECTIONS[dir_index].0);
        let y1 = y.checked_add_signed(DIRECTIONS[dir_index].1);

        // exits the map
        if y1.is_none_or(|y| y >= grid.height) || x1.is_none_or(|x| x >= grid.width) {
            break;
        }
        let (x1, y1) = (x1.unwrap(), y1.unwrap());

        if let Some(ch) = grid.map.get(&(x1, y1)) {
            match ch {
                '.' => {
                    x = x1;
                    y = y1;
                    visited.insert((x1, y1));
                }
                '#' => {
                    dir_index = (dir_index + 1) % 4;
                }
                _ => unimplemented!("UNKNOWN char"),
            }
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, start) = parse(input);

    let mut count = 0;
    let mut movements: HashSet<(usize, usize, usize)> = HashSet::with_capacity(10000);

    for (c, _) in grid.map.clone().iter().filter(|(_, ch)| **ch == '.') {
        grid.map.insert(*c, '#');
        let mut dir_index = 0;
        let (mut x, mut y) = start;
        movements.clear();
        movements.insert((x, y, dir_index));

        loop {
            let x1 = x.checked_add_signed(DIRECTIONS[dir_index].0);
            let y1 = y.checked_add_signed(DIRECTIONS[dir_index].1);

            // exits the map
            if y1.is_none_or(|y| y >= grid.height) || x1.is_none_or(|x| x >= grid.width) {
                break;
            }
            let (x1, y1) = (x1.unwrap(), y1.unwrap());

            if movements.contains(&(x1, y1, dir_index)) {
                count += 1;
                break;
            }

            if let Some(ch) = grid.map.get(&(x1, y1)) {
                match ch {
                    '.' | '^' => {
                        x = x1;
                        y = y1;
                        movements.insert((x1, y1, dir_index));
                    }
                    '#' => {
                        dir_index = (dir_index + 1) % 4;
                    }
                    _ => unimplemented!("UNKNOWN char"),
                }
            }
        }
        grid.map.insert(*c, '.');
    }

    Some(count)
}
