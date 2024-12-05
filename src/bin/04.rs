use itertools::Itertools;

advent_of_code::solution!(4);

fn get_directions() -> Vec<(isize, isize)> {
    vec![
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ]
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_in_dir(
    input: &Vec<Vec<char>>,
    (y_orig, x_orig): &(usize, usize),
    (dy, dx): &(isize, isize),
    word_i: &usize,
) -> Option<()> {
    let word = vec!['X', 'M', 'A', 'S'];

    let mut y = y_orig.to_owned();
    let mut x = x_orig.to_owned();
    let mut i = word_i.to_owned();

    loop {
        let y_next = y.checked_add_signed(*dy);
        let x_next = x.checked_add_signed(*dx);
        let ch = y_next.and_then(|y| {
            input
                .get(y)
                .and_then(|r| x_next.and_then(|x| r.get(x).cloned()))
        });

        if ch != Some(word[i]) {
            break;
        }

        if i == word.len() - 1 {
            return Some(());
        }

        y = y_next.unwrap();
        x = x_next.unwrap();
        i += 1;
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let directions = get_directions();
    let mut count = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char != 'X' {
                continue;
            }
            for (dy, dx) in directions.iter() {
                if find_in_dir(&input, &(y, x), &(*dy, *dx), &1).is_some() {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

fn get_point(
    input: &Vec<Vec<char>>,
    (y, x): (usize, usize),
    (dy, dx): (isize, isize),
) -> Option<char> {
    let y_next = y.checked_add_signed(dy);
    let x_next = x.checked_add_signed(dx);
    y_next.and_then(|y| {
        input
            .get(y)
            .and_then(|r| x_next.and_then(|x| r.get(x).cloned()))
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let directions = get_directions();
    let mut count = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char != 'A' {
                continue;
            }

            let left = vec![directions[7], directions[3]]
                .iter()
                .map(|dir| get_point(&input, (y, x), *dir))
                .collect_vec();
            let right = vec![directions[1], directions[5]]
                .iter()
                .map(|dir| get_point(&input, (y, x), *dir))
                .collect_vec();

            if left.iter().any(|ch| *ch == Some('M'))
                && left.iter().any(|ch| *ch == Some('S'))
                && right.iter().any(|ch| *ch == Some('M'))
                && right.iter().any(|ch| *ch == Some('S'))
            {
                count += 1;
            }
        }
    }

    Some(count)
}
