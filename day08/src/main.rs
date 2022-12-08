fn parse_input(input: &[&str]) -> Vec<Vec<u8>> {
    let mut rows = Vec::new();
    for line in input {
        rows.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    rows
}

fn solve1(input: &[&str]) -> usize {
    let mut tot_visible = 0;
    let input = parse_input(input);
    for y in 1..input.len() - 1 {
        for x in 1..input[0].len() - 1 {
            let height = input[y][x];
            let mut visible = true;

            // test left
            let mut i = x;
            while i > 0 {
                i -= 1;

                if input[y][i] >= height {
                    visible = false;
                    break;
                }
            }
            if visible {
                tot_visible += 1;
                continue;
            }

            // test right
            visible = true;
            let mut i = x;
            while i < input[0].len() - 1 {
                i += 1;
                if input[y][i] >= height {
                    visible = false;
                    break;
                }
            }
            if visible {
                tot_visible += 1;
                continue;
            }

            // test up
            visible = true;
            let mut i = y;
            while i > 0 {
                i -= 1;
                if input[i][x] >= height {
                    visible = false;
                    break;
                }
            }
            if visible {
                tot_visible += 1;
                continue;
            }

            // test down
            visible = true;
            let mut i = y;
            while i < input.len() - 1 {
                i += 1;

                if input[i][x] >= height {
                    visible = false;
                    break;
                }
            }
            if visible {
                tot_visible += 1;
                continue;
            }
        }
    }

    tot_visible + input.len() * 2 + (input[0].len() - 2) * 2
}

fn solve2(input: &[&str]) -> u32 {
    let mut best_score = 0;
    let input = parse_input(input);
    for (y, row) in input.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let mut left_score = 0;
            if x > 0 {
                for i in (0..x).rev() {
                    left_score += 1;
                    if input[y][i] >= *col {
                        break;
                    }
                }
            }

            let mut right_score = 0;
            if x < row.len() - 1 {
                for i in x + 1..=row.len() - 1 {
                    right_score += 1;
                    if input[y][i] >= *col {
                        break;
                    }
                }
            }

            let mut up_score = 0;
            if y > 0 {
                for i in (0..y).rev() {
                    up_score += 1;
                    if input[i][x] >= *col {
                        break;
                    }
                }
            }

            let mut down_score = 0;
            if y < input.len() - 1 {
                for row in input.iter().skip(y + 1) {
                    down_score += 1;
                    if row[x] >= *col {
                        break;
                    }
                }
            }

            best_score = best_score.max(left_score * right_score * up_score * down_score);
        }
    }

    best_score
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &["30373", "25512", "65332", "33549", "35390"];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 21)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 8)
    }
}
