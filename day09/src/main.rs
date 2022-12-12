use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coords {
    x: i32,
    y: i32,
}

fn solve1(input: &[&str]) -> usize {
    let mut head = Coords { x: 0, y: 0 };
    let mut tail = Coords { x: 0, y: 0 };
    let mut visited = HashSet::new();

    let mut prev_head = head;

    for line in input {
        let (dir, num) = line.split_once(' ').unwrap();
        for _ in 0..num.parse().unwrap() {
            match dir {
                "U" => head.y += 1,
                "R" => head.x += 1,
                "D" => head.y -= 1,
                "L" => head.x -= 1,
                _ => panic!("invalid direction {dir}"),
            }

            let distance = (head.x - tail.x).abs() + (head.y - tail.y).abs();
            // check if diagonal from tail, distance 2
            if head.x != tail.x && head.y != tail.y {
                if distance > 2 {
                    tail = prev_head;
                }
            } else if distance == 2 {
                tail = prev_head
            }

            visited.insert(tail);
            prev_head = head;
        }
    }

    visited.len()
}

fn solve2(input: &[&str]) -> usize {
    fn fix_sub_knots(knots: &mut [Coords], start_idx: usize) {
        assert!(start_idx > 0);
        if start_idx >= knots.len() {
            return;
        }

        let head = knots[start_idx - 1];
        let mut new_tail = knots[start_idx];

        loop {
            let distance = (head.x - new_tail.x).abs() + (head.y - new_tail.y).abs();

            // check if diagonal from tail, distance 2
            if head.x != new_tail.x && head.y != new_tail.y {
                if distance <= 2 {
                    break;
                } else {
                    // diagonal move
                    if head.x > new_tail.x {
                        new_tail.x += 1;
                    } else {
                        new_tail.x -= 1;
                    }

                    if head.y > new_tail.y {
                        new_tail.y += 1;
                    } else {
                        new_tail.y -= 1;
                    }
                }
            } else if distance <= 1 {
                break;
            } else {
                // simple move
                #[allow(clippy::comparison_chain)]
                if head.x == new_tail.x {
                    if head.y > new_tail.y {
                        new_tail.y += 1;
                    } else {
                        new_tail.y -= 1;
                    }
                } else if head.x > new_tail.x {
                    new_tail.x += 1;
                } else {
                    new_tail.x -= 1;
                }
            }

            knots[start_idx] = new_tail;

            fix_sub_knots(knots, start_idx + 1);
        }
    }

    let num_knots = 10;
    let mut knots = vec![Coords { x: 0, y: 0 }; num_knots];
    let mut visited = HashSet::new();

    for line in input {
        let (dir, num) = line.split_once(' ').unwrap();
        for _ in 0..num.parse().unwrap() {
            match dir {
                "U" => knots[0].y += 1,
                "R" => knots[0].x += 1,
                "D" => knots[0].y -= 1,
                "L" => knots[0].x -= 1,
                _ => panic!("invalid direction {dir}"),
            }

            fix_sub_knots(&mut knots, 1);

            visited.insert(knots[num_knots - 1]);
        }
    }

    visited.len()
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 13)
    }

    #[test]
    fn test2() {
        // assert_eq!(solve2(INPUT), 1);
        let input = &["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];
        assert_eq!(solve2(input), 36)
    }
}
