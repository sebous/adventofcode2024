use std::collections::{HashMap, HashSet};

use advent_of_code::grid::{Coord, Grid};

advent_of_code::solution!(6);

const directions: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

const guard: [char; 4] = ['^', '>', '⌄', '<'];

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
    let (mut grid, start) = parse(input);

    let (mut x, mut y) = start;
    let mut dir_index = 0;
    let mut movements = HashSet::from([(x, y)]);

    loop {
        let x1 = x.checked_add_signed(directions[dir_index].0);
        let y1 = y.checked_add_signed(directions[dir_index].1);

        // exits the map
        if y1.is_none_or(|y| y >= grid.height) || x1.is_none_or(|x| x >= grid.width) {
            break;
        }
        let (x1, y1) = (x1.unwrap(), y1.unwrap());

        if let Some(ch) = grid.map.get(&(x1, y1)) {
            match ch {
                '.' => {
                    grid.map.insert((x, y), '.');
                    grid.map.insert((x1, y1), guard[dir_index]);
                    x = x1;
                    y = y1;
                    movements.insert((x1, y1));
                }
                '#' => {
                    dir_index = (dir_index + 1) % 4;
                    grid.map.insert((x, y), guard[dir_index]);
                }
                _ => unimplemented!("UNKNOWN char"),
            }
        }
        // println!("{}", grid);
    }

    Some(movements.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
