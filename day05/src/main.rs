use std::collections::HashMap;

#[derive(Debug)]
struct Supplies {
    stacks: Vec<Vec<char>>,
}

struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Supplies {
    fn new(input: &[&str]) -> Self {
        let mut input = input.iter().rev();
        let cols = input.next().unwrap();
        let mut col_idx = HashMap::new();
        for (i, col) in cols.chars().enumerate() {
            if col.is_numeric() {
                col_idx.insert(i, (col.to_digit(10).unwrap() - 1) as usize);
            }
        }
        let mut stacks = vec![Vec::new(); col_idx.len()];
        for line in input {
            for (i, chr) in line.chars().enumerate() {
                if chr.is_alphabetic() {
                    stacks[*(col_idx.get(&i).unwrap())].push(chr);
                }
            }
        }

        Self { stacks }
    }
}

impl Move {
    fn new(input: &str) -> Self {
        let parts: Vec<usize> = input
            .split_whitespace()
            .map(|c| c.parse::<usize>())
            .filter_map(|s| s.ok())
            .collect();

        Self {
            quantity: parts[0],
            from: parts[1] - 1,
            to: parts[2] - 1,
        }
    }
}

fn parse_input(input: &[&str]) -> (Supplies, Vec<Move>) {
    let mut parts = input.split(|line| line.is_empty());
    let supplies = parts.next().unwrap();
    let moves = parts.next().unwrap();

    let supplies = Supplies::new(supplies);
    let moves: Vec<Move> = moves.iter().map(|m| Move::new(m)).collect();

    (supplies, moves)
}

fn solve1(input: &[&str]) -> String {
    let (mut supplies, moves) = parse_input(input);
    for step in moves {
        for _ in 0..step.quantity {
            let moved_box = supplies.stacks[step.from].pop().unwrap();
            supplies.stacks[step.to].push(moved_box);
        }
    }

    String::from_iter(
        supplies
            .stacks
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s[s.len() - 1]),
    )
}

fn solve2(input: &[&str]) -> String {
    let (mut supplies, moves) = parse_input(input);
    for step in moves {
        let range = supplies.stacks[step.from].len() - step.quantity..;
        let boxes: Vec<char> = supplies.stacks[step.from].drain(range).collect();
        for mbox in boxes {
            supplies.stacks[step.to].push(mbox);
        }
    }

    String::from_iter(
        supplies
            .stacks
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s[s.len() - 1]),
    )
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT).as_str(), "CMZ")
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT).as_str(), "MCD")
    }
}
