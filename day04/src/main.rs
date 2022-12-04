fn solve1(input: &[&str]) -> u32 {
    let mut overlaps = 0;
    for line in input {
        let (left, right) = line.split_once(',').unwrap();
        let (left_left, left_right) = left.split_once('-').unwrap();
        let (right_left, right_right) = right.split_once('-').unwrap();
        let left = left_left.parse::<u32>().unwrap()..=left_right.parse::<u32>().unwrap();
        let right = right_left.parse::<u32>().unwrap()..=right_right.parse::<u32>().unwrap();

        if (left.contains(right.start()) && left.contains(right.end()))
            || (right.contains(left.start()) && right.contains(left.end()))
        {
            overlaps += 1;
        }
    }

    overlaps
}

fn solve2(input: &[&str]) -> u32 {
    let mut overlaps = 0;
    for line in input {
        let (left, right) = line.split_once(',').unwrap();
        let (left_left, left_right) = left.split_once('-').unwrap();
        let (right_left, right_right) = right.split_once('-').unwrap();
        let left = left_left.parse::<u32>().unwrap()..=left_right.parse::<u32>().unwrap();
        let right = right_left.parse::<u32>().unwrap()..=right_right.parse::<u32>().unwrap();

        if left.contains(right.start())
            || left.contains(right.end())
            || right.contains(left.start())
            || right.contains(left.end())
        {
            overlaps += 1;
        }
    }

    overlaps
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
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 2)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 4)
    }
}
