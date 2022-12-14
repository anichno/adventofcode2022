use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Int(i32),
    List(VecDeque<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match compare(self.clone(), other.clone()) {
            Order::Correct => std::cmp::Ordering::Less,
            Order::Incorrect => std::cmp::Ordering::Greater,
            Order::Unknown => std::cmp::Ordering::Equal,
        }
    }
}

#[derive(Debug)]
enum Order {
    Correct,
    Incorrect,
    Unknown,
}

fn parse_line(input: &mut std::str::Chars) -> Value {
    let mut vals = VecDeque::new();
    let mut cur_num = String::new();
    while let Some(next_char) = input.next() {
        if next_char == '[' {
            vals.push_back(parse_line(input));
        } else if next_char.is_ascii_digit() {
            cur_num.push(next_char);
        } else if next_char == ',' {
            if !cur_num.is_empty() {
                vals.push_back(Value::Int(cur_num.parse().unwrap()));
                cur_num.clear();
            }
        } else if next_char == ']' {
            if !cur_num.is_empty() {
                vals.push_back(Value::Int(cur_num.parse().unwrap()));
                cur_num.clear();
            }
            return Value::List(vals);
        }
    }

    unreachable!()
}

fn parse_input(input: &[&str]) -> Vec<(Value, Value)> {
    let mut pairs = Vec::new();
    let mut input = input.iter();
    loop {
        // left
        let line = input.next().unwrap();
        let mut char_iter = line.chars();
        // consume first '['
        char_iter.next().unwrap();

        let left = parse_line(&mut char_iter);

        // right
        let line = input.next().unwrap();
        let mut char_iter = line.chars();
        // consume first '['
        char_iter.next().unwrap();

        let right = parse_line(&mut char_iter);

        pairs.push((left, right));

        // blank line
        if input.next().is_none() {
            break;
        }
    }

    pairs
}

fn compare(left: Value, right: Value) -> Order {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => match left.cmp(&right) {
            std::cmp::Ordering::Less => return Order::Correct,
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => return Order::Incorrect,
        },
        (Value::Int(left), Value::List(right)) => {
            return compare(
                Value::List(vec![Value::Int(left)].into()),
                Value::List(right),
            )
        }
        (Value::List(left), Value::Int(right)) => {
            return compare(
                Value::List(left),
                Value::List(vec![Value::Int(right)].into()),
            )
        }
        (Value::List(mut left), Value::List(mut right)) => loop {
            let left_val = left.pop_front();
            let right_val = right.pop_front();
            match (left_val, right_val) {
                (None, None) => break,
                (None, Some(_)) => return Order::Correct,
                (Some(_), None) => return Order::Incorrect,
                (Some(left_val), Some(right_val)) => {
                    let result = compare(left_val, right_val);
                    if !matches!(result, Order::Unknown) {
                        return result;
                    }
                }
            }
        },
    }
    Order::Unknown
}

fn solve1(input: &[&str]) -> usize {
    let pairs = parse_input(input);
    let mut result = 0;
    for (idx, (left, right)) in pairs.into_iter().enumerate() {
        if let Order::Correct = compare(left, right) {
            result += idx + 1;
        }
    }
    result
}

fn solve2(input: &[&str]) -> usize {
    let mut packets: Vec<Value> = Vec::new();
    for (left, right) in parse_input(input) {
        packets.push(left);
        packets.push(right);
    }
    let (divider_packet1, divider_packet2) = parse_input(&["[[2]]", "[[6]]"]).pop().unwrap();
    packets.push(divider_packet1.clone());
    packets.push(divider_packet2.clone());

    packets.sort_unstable();

    let mut divider1_idx = None;
    let mut divider2_idx = None;

    for (idx, packet) in packets.into_iter().enumerate() {
        if packet == divider_packet1 {
            divider1_idx = Some(idx + 1);
        } else if packet == divider_packet2 {
            divider2_idx = Some(idx + 1);
        }
    }

    if let (Some(idx1), Some(idx2)) = (divider1_idx, divider2_idx) {
        idx1 * idx2
    } else {
        panic!()
    }
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
        "[1,1,3,1,1]",
        "[1,1,5,1,1]",
        "",
        "[[1],[2,3,4]]",
        "[[1],4]",
        "",
        "[9]",
        "[[8,7,6]]",
        "",
        "[[4,4],4,4]",
        "[[4,4],4,4,4]",
        "",
        "[7,7,7,7]",
        "[7,7,7]",
        "",
        "[]",
        "[3]",
        "",
        "[[[]]]",
        "[[]]",
        "",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 13)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 140)
    }
}
