use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Node {
    Const(usize),
    Add((Box<Node>, Box<Node>)),
    Mul((Box<Node>, Box<Node>)),
    Old,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        match value {
            "old" => Node::Old,
            _ => Node::Const(value.parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Node,
    div_test: usize,
    true_target: usize,
    false_target: usize,
}

fn parse_input(input: &[&str]) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let mut lines = input.iter();
    let mut cur_line = lines.next();
    while let Some(_line) = cur_line {
        // Monkey

        // Starting items
        let (_, items) = lines.next().unwrap().split_once(':').unwrap();
        let items = items
            .trim()
            .split(", ")
            .map(|i| i.trim().parse().unwrap())
            .collect();

        // Operation
        let (_, op_txt) = lines.next().unwrap().split_once('=').unwrap();
        let mut parts = op_txt.split_whitespace();
        let left: Node = parts.next().unwrap().into();
        let op = parts.next().unwrap();
        let right: Node = parts.next().unwrap().into();
        let operation = match op {
            "+" => Node::Add((Box::new(left), Box::new(right))),
            "*" => Node::Mul((Box::new(left), Box::new(right))),
            _ => panic!("invalid op {op}"),
        };

        // Test
        let (_, div_test) = lines.next().unwrap().split_once("by").unwrap();
        let div_test = div_test.trim().parse().unwrap();

        // True target
        let (_, true_target) = lines.next().unwrap().split_once("monkey").unwrap();
        let true_target = true_target.trim().parse().unwrap();

        // False target
        let (_, false_target) = lines.next().unwrap().split_once("monkey").unwrap();
        let false_target = false_target.trim().parse().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            div_test,
            true_target,
            false_target,
        });

        // empty trailing line
        lines.next();
        cur_line = lines.next();
    }

    monkeys
}

fn solve1(mut input: Vec<Monkey>) -> u32 {
    let mut monkey_counter = vec![0; input.len()];

    for _round in 0..20 {
        for monkey_id in 0..input.len() {
            while !input[monkey_id].items.is_empty() {
                monkey_counter[monkey_id] += 1;
                let cur_item = input[monkey_id].items.pop_front();
                let Some(cur_item) = cur_item else {
                continue;
            };

                let mut worry = cur_item;
                worry = match &input[monkey_id].operation {
                    Node::Const(val) => *val,
                    Node::Add((left, right)) => {
                        let left = match **left {
                            Node::Const(val) => val,
                            Node::Add(_) => panic!(),
                            Node::Mul(_) => panic!(),
                            Node::Old => worry,
                        };
                        let right = match **right {
                            Node::Const(val) => val,
                            Node::Add(_) => panic!(),
                            Node::Mul(_) => panic!(),
                            Node::Old => worry,
                        };
                        left + right
                    }
                    Node::Mul((left, right)) => {
                        let left = match **left {
                            Node::Const(val) => val,
                            Node::Add(_) => panic!(),
                            Node::Mul(_) => panic!(),
                            Node::Old => worry,
                        };
                        let right = match **right {
                            Node::Const(val) => val,
                            Node::Add(_) => panic!(),
                            Node::Mul(_) => panic!(),
                            Node::Old => worry,
                        };
                        left * right
                    }
                    Node::Old => todo!(),
                };

                // worry decay
                worry /= 3;

                if worry % input[monkey_id].div_test == 0 {
                    let true_target = input[monkey_id].true_target;
                    input[true_target].items.push_back(worry);
                } else {
                    let false_target = input[monkey_id].false_target;
                    input[false_target].items.push_back(worry);
                }
            }
        }
    }
    monkey_counter.sort_unstable();
    monkey_counter
        .iter()
        .rev()
        .take(2)
        .copied()
        .reduce(|acc, i| acc * i)
        .unwrap()
}

fn solve2(mut input: Vec<Monkey>) -> usize {
    let mut monkey_counter = vec![0; input.len()];
    let big_div = input
        .iter()
        .map(|i| i.div_test)
        .reduce(|acc, i| acc * i)
        .unwrap();
    println!("{big_div}");

    for _round in 0..10000 {
        for monkey_id in 0..input.len() {
            while !input[monkey_id].items.is_empty() {
                monkey_counter[monkey_id] += 1;
                let cur_item = input[monkey_id].items.pop_front();
                let Some(cur_item) = cur_item else {
                continue;
            };

                let mut worry = cur_item;
                worry = match &input[monkey_id].operation {
                    Node::Const(val) => *val,
                    Node::Add((left, right)) => {
                        let left = match **left {
                            Node::Const(val) => val,
                            Node::Add(_) => panic!(),
                            Node::Mul(_) => panic!(),
                            Node::Old => worry,
                        };
                        let right = match **right {
                            Node::Const(val) => val,
                            Node::Add(_) => panic!(),
                            Node::Mul(_) => panic!(),
                            Node::Old => worry,
                        };
                        left + right
                    }
                    Node::Mul((left, right)) => {
                        let left = match **left {
                            Node::Const(val) => val,
                            Node::Add(_) => panic!(),
                            Node::Mul(_) => panic!(),
                            Node::Old => worry,
                        };
                        let right = match **right {
                            Node::Const(val) => val,
                            Node::Add(_) => panic!(),
                            Node::Mul(_) => panic!(),
                            Node::Old => worry,
                        };
                        left * right
                    }
                    Node::Old => todo!(),
                };

                // worry decay
                worry %= big_div;

                if worry % input[monkey_id].div_test == 0 {
                    let true_target = input[monkey_id].true_target;
                    input[true_target].items.push_back(worry);
                } else {
                    let false_target = input[monkey_id].false_target;
                    input[false_target].items.push_back(worry);
                }
            }
        }
    }

    monkey_counter.sort_unstable();
    monkey_counter
        .iter()
        .rev()
        .take(2)
        .copied()
        .reduce(|acc, i| acc * i)
        .unwrap()
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let input = parse_input(&input);

    println!("part 1: {}", solve1(input.clone()));
    println!("part 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Monkey 0:",
        "  Starting items: 79, 98",
        "  Operation: new = old * 19",
        "  Test: divisible by 23",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 3",
        "",
        "Monkey 1:",
        "  Starting items: 54, 65, 75, 74",
        "  Operation: new = old + 6",
        "  Test: divisible by 19",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 0",
        "",
        "Monkey 2:",
        "  Starting items: 79, 60, 97",
        "  Operation: new = old * old",
        "  Test: divisible by 13",
        "    If true: throw to monkey 1",
        "    If false: throw to monkey 3",
        "",
        "Monkey 3:",
        "  Starting items: 74",
        "  Operation: new = old + 3",
        "  Test: divisible by 17",
        "    If true: throw to monkey 0",
        "    If false: throw to monkey 1",
    ];

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT);
        assert_eq!(solve1(parsed), 10605)
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT);
        assert_eq!(solve2(parsed), 2713310158)
    }
}
