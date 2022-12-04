use std::collections::HashSet;

fn get_priority(chr: u8) -> u32 {
    if (97..=122).contains(&chr) {
        (chr - 96) as u32
    } else if (65..=90).contains(&chr) {
        (chr - 38) as u32
    } else {
        panic!("invalid chr")
    }
}

fn solve1(input: &[&str]) -> u32 {
    let mut tot_priority = 0;
    for line in input {
        let input = line.as_bytes();
        let left: HashSet<u8> = HashSet::from_iter(input[..input.len() / 2].iter().copied());
        let right: HashSet<u8> = HashSet::from_iter(input[input.len() / 2..].iter().copied());
        for chr in left.intersection(&right) {
            tot_priority += get_priority(*chr);
        }
    }

    tot_priority
}

fn solve2(input: &[&str]) -> u32 {
    let mut tot_priority = 0;
    for group in input.chunks(3) {
        let mut common_items: Option<HashSet<u8>> = None;
        for line in group {
            let input = line.as_bytes();
            let pack: HashSet<u8> = HashSet::from_iter(input.iter().copied());
            if let Some(common) = &mut common_items {
                *common = HashSet::from_iter(common.intersection(&pack).copied());
            } else {
                common_items = Some(pack);
            }
        }
        let Some(common) = common_items else {
            panic!("nothing in common");
        };

        for chr in common {
            tot_priority += get_priority(chr);
        }
    }

    tot_priority
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
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 157)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 70)
    }
}
