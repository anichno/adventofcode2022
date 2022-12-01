fn solve1(input: &[&str]) -> u32 {
    let mut max_elf = 0;
    let mut cur_elf = 0;
    for line in input {
        if line.is_empty() {
            max_elf = max_elf.max(cur_elf);
            cur_elf = 0;
            continue;
        }

        cur_elf += line.parse::<u32>().unwrap()
    }

    max_elf
}

fn solve2(input: &[&str]) -> u32 {
    let mut elves = Vec::new();
    let mut cur_elf = 0;
    for line in input {
        if line.is_empty() {
            elves.push(cur_elf);
            cur_elf = 0;
            continue;
        }

        cur_elf += line.parse::<u32>().unwrap()
    }
    elves.push(cur_elf);

    elves.sort_unstable();
    let mut top_3_cals = 0;
    for _ in 0..3 {
        top_3_cals += elves.pop().unwrap();
    }

    top_3_cals
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
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 24000)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 45000)
    }
}
