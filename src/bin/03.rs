advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re_ops = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res = re_ops.captures_iter(input).fold(0, |total, c| {
        let x = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let y = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
        total + x * y
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_ops =
        Regex::new(r"(?<mul>mul\((?<x>\d+),(?<y>\d+)\))|(?<dont>don't\(\))|(?<do>do\(\))").unwrap();

    let mut enabled = true;

    let result = re_ops.captures_iter(input).fold(0, |total, c| {
        if c.name("do").is_some() {
            enabled = true;
        }

        if c.name("dont").is_some() {
            enabled = false;
        }

        if c.name("mul").is_some() && enabled {
            let x = c.name("x").unwrap().as_str().parse::<u32>().unwrap();
            let y = c.name("y").unwrap().as_str().parse::<u32>().unwrap();
            return total + x * y;
        }

        return total;
    });

    Some(result)
}
