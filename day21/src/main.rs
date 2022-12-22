mod part1 {
    use std::collections::HashMap;

    pub enum Node {
        Constant(i64),
        Op(Box<Operation>),
    }

    pub enum Operation {
        Add((Node, Node)),
        Sub((Node, Node)),
        Mul((Node, Node)),
        Div((Node, Node)),
    }

    fn build_tree(cur_name: &str, map: &HashMap<&str, &str>) -> Node {
        let line = map.get(cur_name).unwrap();
        if let Ok(num) = line.parse() {
            Node::Constant(num)
        } else {
            let (name1, line) = line.split_once(' ').unwrap();
            let (op, name2) = line.split_once(' ').unwrap();

            let left_node = build_tree(name1, map);
            let right_node = build_tree(name2, map);
            match op {
                "+" => Node::Op(Box::new(Operation::Add((left_node, right_node)))),
                "-" => Node::Op(Box::new(Operation::Sub((left_node, right_node)))),
                "*" => Node::Op(Box::new(Operation::Mul((left_node, right_node)))),
                "/" => Node::Op(Box::new(Operation::Div((left_node, right_node)))),
                _ => panic!("invalid op: {op}"),
            }
        }
    }

    pub fn parse_input(input: &[&str]) -> Node {
        let mut map = HashMap::new();
        for line in input {
            let (name, line) = line.split_once(": ").unwrap();
            map.insert(name, line);
        }

        build_tree("root", &map)
    }
}

mod part2 {
    use std::collections::HashMap;

    pub enum Node {
        Constant(i64),
        Me,
        Op(Box<Operation>),
    }

    pub enum Operation {
        Add((Node, Node)),
        Sub((Node, Node)),
        Mul((Node, Node)),
        Div((Node, Node)),
        Eq((Node, Node)),
    }

    fn build_tree(cur_name: &str, map: &HashMap<&str, &str>) -> Node {
        if cur_name == "humn" {
            Node::Me
        } else {
            let line = map.get(cur_name).unwrap();
            if let Ok(num) = line.parse() {
                Node::Constant(num)
            } else {
                let (name1, line) = line.split_once(' ').unwrap();
                let (op, name2) = line.split_once(' ').unwrap();

                let left_node = build_tree(name1, map);
                let right_node = build_tree(name2, map);
                if cur_name == "root" {
                    Node::Op(Box::new(Operation::Eq((left_node, right_node))))
                } else {
                    match op {
                        "+" => Node::Op(Box::new(Operation::Add((left_node, right_node)))),
                        "-" => Node::Op(Box::new(Operation::Sub((left_node, right_node)))),
                        "*" => Node::Op(Box::new(Operation::Mul((left_node, right_node)))),
                        "/" => Node::Op(Box::new(Operation::Div((left_node, right_node)))),
                        _ => panic!("invalid op: {op}"),
                    }
                }
            }
        }
    }

    pub fn parse_input(input: &[&str]) -> Node {
        let mut map = HashMap::new();
        for line in input {
            let (name, line) = line.split_once(": ").unwrap();
            map.insert(name, line);
        }

        build_tree("root", &map)
    }
}

fn solve1(input: &[&str]) -> i64 {
    use part1::{parse_input, Node, Operation};
    fn eval(cur_node: &part1::Node) -> i64 {
        match cur_node {
            Node::Constant(val) => *val,
            Node::Op(op) => match op.as_ref() {
                Operation::Add((left, right)) => eval(left) + eval(right),
                Operation::Sub((left, right)) => eval(left) - eval(right),
                Operation::Mul((left, right)) => eval(left) * eval(right),
                Operation::Div((left, right)) => eval(left) / eval(right),
            },
        }
    }
    let tree = parse_input(input);
    eval(&tree)
}

fn solve2(input: &[&str]) -> i64 {
    use part2::{parse_input, Node, Operation};
    use z3::{
        ast::{Ast, Int},
        Config, Context,
    };

    fn eval<'a>(cur_node: &Node, me_var: &'a Int, context: &'a Context) -> Int<'a> {
        match cur_node {
            Node::Constant(val) => Int::from_i64(context, *val),
            Node::Op(op) => match op.as_ref() {
                Operation::Add((left, right)) => {
                    eval(left, me_var, context) + eval(right, me_var, context)
                }
                Operation::Sub((left, right)) => {
                    eval(left, me_var, context) - eval(right, me_var, context)
                }
                Operation::Mul((left, right)) => {
                    eval(left, me_var, context) * eval(right, me_var, context)
                }
                Operation::Div((left, right)) => {
                    eval(left, me_var, context) / eval(right, me_var, context)
                }
                Operation::Eq(_) => todo!(),
            },
            Node::Me => me_var.clone(),
        }
    }
    let tree = parse_input(input);
    let mut config = Config::new();
    config.set_model_generation(true);
    let context = Context::new(&config);
    let solver = z3::Solver::new(&context);

    let me_var = Int::new_const(&context, "me");
    let Node::Op(tree) = tree else {
        panic!();
    };
    let Operation::Eq((left, right)) = tree.as_ref() else {
        panic!();
    };

    let left = eval(left, &me_var, &context);
    let right = eval(right, &me_var, &context);
    solver.assert(&left._eq(&right));

    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            model.eval(&me_var, true).unwrap().as_i64().unwrap()
        }
        _ => panic!("failed to sat"),
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
        "root: pppw + sjmn",
        "dbpl: 5",
        "cczh: sllz + lgvd",
        "zczc: 2",
        "ptdq: humn - dvpt",
        "dvpt: 3",
        "lfqf: 4",
        "humn: 5",
        "ljgn: 2",
        "sjmn: drzm * dbpl",
        "sllz: 4",
        "pppw: cczh / lfqf",
        "lgvd: ljgn * ptdq",
        "drzm: hmdt - zczc",
        "hmdt: 32",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 152)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 301)
    }
}