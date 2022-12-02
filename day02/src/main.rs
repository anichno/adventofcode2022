fn solve1(input: &[&str]) -> u32 {
    let mut score_tot = 0;
    for line in input {
        let Some((left, right)) = line.split_once(' ') else {
            continue;
        };

        let mut score = 0;
        score += match right {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("invalid char"),
        };

        score += match left {
            "A" => match right {
                "X" => 3,
                "Y" => 6,
                "Z" => 0,
                _ => panic!("invalid char"),
            },
            "B" => match right {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!("invalid char"),
            },
            "C" => match right {
                "X" => 6,
                "Y" => 0,
                "Z" => 3,
                _ => panic!("invalid char"),
            },
            _ => panic!("invalid char"),
        };

        score_tot += score;
    }

    score_tot
}

#[allow(clippy::identity_op)]
fn solve2(input: &[&str]) -> u32 {
    let mut score_tot = 0;
    for line in input {
        let Some((left, right)) = line.split_once(' ') else {
            continue;
        };

        score_tot += match left {
            "A" => match right {
                "X" => 0 + 3,
                "Y" => 3 + 1,
                "Z" => 6 + 2,
                _ => panic!("invalid char"),
            },
            "B" => match right {
                "X" => 0 + 1,
                "Y" => 3 + 2,
                "Z" => 6 + 3,
                _ => panic!("invalid char"),
            },
            "C" => match right {
                "X" => 0 + 2,
                "Y" => 3 + 3,
                "Z" => 6 + 1,
                _ => panic!("invalid char"),
            },
            _ => panic!("invalid char"),
        };
    }

    score_tot
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &["A Y", "B X", "C Z"];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 15)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 12)
    }
}
