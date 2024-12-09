advent_of_code::solution!(7);

struct Equation {
    total: u64,
    values: Vec<u64>,
}

#[derive(PartialEq, Eq)]
enum Op {
    Add,
    Multiply,
    Concat,
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let (total, rest) = l.split_once(": ").unwrap();
            Equation {
                total: total.parse().unwrap(),
                values: rest
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn permutations_with_repetition<T>(items: &[T], k: usize) -> Vec<Vec<&T>> {
    let mut result = vec![vec![]];

    for _ in 0..k {
        let mut new_result = Vec::new();
        for r in result {
            for item in items {
                let mut temp = r.clone();
                temp.push(item);
                new_result.push(temp);
            }
        }
        result = new_result;
    }
    result
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn solve(eqs: Vec<Equation>, ops: Vec<Op>) -> u64 {
    let mut test_values_sum = 0;

    'main: for eq in &eqs {
        let permutations = permutations_with_repetition(&ops, eq.values.len() - 1);
        for perm in permutations {
            let perm_total = eq.values.iter().enumerate().fold(0, |total, (i, val)| {
                if i == 0 {
                    return val.to_owned();
                }

                match perm[i - 1] {
                    Op::Add => total + val,
                    Op::Multiply => total * val,
                    Op::Concat => concat(total, *val),
                }
            });

            if perm_total == eq.total {
                test_values_sum += eq.total;
                continue 'main;
            }
        }
    }
    test_values_sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let eqs = parse(input);
    let ops = vec![Op::Add, Op::Multiply];
    let sum = solve(eqs, ops);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let eqs = parse(input);
    let ops = vec![Op::Add, Op::Multiply, Op::Concat];
    let sum = solve(eqs, ops);
    Some(sum)
}
