fn solve1(input: &[&str]) -> i64 {
    let initial: Vec<(usize, i64)> = input
        .iter()
        .enumerate()
        .map(|(i, n)| (i, n.parse().unwrap()))
        .collect();
    let mut progress = initial.clone();
    let file_len = initial.len();
    for (i, val) in initial {
        let pos = progress
            .iter()
            .position(|(j, v)| *j == i && *v == val)
            .unwrap();
        let new_pos = (pos as i64 + val).rem_euclid(file_len as i64 - 1);

        progress.remove(pos);
        progress.insert(new_pos as usize, (i, val));
    }

    let zero_idx = progress.iter().position(|(_, v)| *v == 0).unwrap();
    progress[(zero_idx + 1000) % file_len].1
        + progress[(zero_idx + 2000) % file_len].1
        + progress[(zero_idx + 3000) % file_len].1
}

fn solve2(input: &[&str]) -> i64 {
    let initial: Vec<(usize, i64)> = input
        .iter()
        .enumerate()
        .map(|(i, n)| (i, n.parse::<i64>().unwrap() * 811589153))
        .collect();
    let mut progress = initial.clone();
    let file_len = initial.len();
    for _round in 0..10 {
        for (i, val) in initial.iter().copied() {
            let pos = progress
                .iter()
                .position(|(j, v)| *j == i && *v == val)
                .unwrap();
            let new_pos = (pos as i64 + val).rem_euclid(file_len as i64 - 1);

            progress.remove(pos);
            progress.insert(new_pos as usize, (i, val));
        }
    }

    let zero_idx = progress.iter().position(|(_, v)| *v == 0).unwrap();
    progress[(zero_idx + 1000) % file_len].1
        + progress[(zero_idx + 2000) % file_len].1
        + progress[(zero_idx + 3000) % file_len].1
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &["1", "2", "-3", "3", "-2", "0", "4"];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 3)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 1623178306)
    }
}
