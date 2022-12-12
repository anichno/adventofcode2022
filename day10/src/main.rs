fn solve1(input: &[&str]) -> i32 {
    let mut cycle = 1;
    let mut reg_x: i32 = 1;

    let mut instructions = input.iter();

    let checkpoints = &[20, 60, 100, 140, 180, 220];
    let mut checkpoint_x = Vec::new();

    let mut busy = 0;
    let mut x_effect = 0;
    while cycle <= 220 {
        if busy == 0 {
            reg_x += x_effect;
            x_effect = 0;
            let mut parts = instructions.next().unwrap().split_whitespace();
            let opcode = parts.next().unwrap();
            match opcode {
                "noop" => busy = 1,
                "addx" => {
                    busy = 2;
                    x_effect = parts.next().unwrap().parse().unwrap();
                }
                _ => panic!("invalid opcode: {opcode}"),
            }
        }
        if checkpoints.contains(&cycle) {
            checkpoint_x.push(reg_x * cycle);
        }

        cycle += 1;
        busy -= 1;
    }

    checkpoint_x.iter().sum()
}

fn solve2(input: &[&str]) -> String {
    let mut screen = vec![vec![false; 40]; 6];

    let mut cycle = 0;
    let mut reg_x: i32 = 1;

    let mut instructions = input.iter();

    let mut busy = 0;
    let mut x_effect = 0;
    while cycle < 240 {
        if busy == 0 {
            reg_x += x_effect;
            x_effect = 0;
            let mut parts = instructions.next().unwrap().split_whitespace();
            let opcode = parts.next().unwrap();
            match opcode {
                "noop" => busy = 1,
                "addx" => {
                    busy = 2;
                    x_effect = parts.next().unwrap().parse().unwrap();
                }
                _ => panic!("invalid opcode: {opcode}"),
            }
        }

        let cur_row = (cycle / 40) as usize;
        let cur_col = (cycle % 40) as usize;

        if (reg_x - 1..=reg_x + 1).contains(&(cur_col as i32)) {
            screen[cur_row][cur_col] = true;
        }

        cycle += 1;
        busy -= 1;
    }

    let mut output = String::new();
    for row in screen {
        for col in row {
            if col {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }

    output
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: \n{}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("testcase.txt");

    #[test]
    fn test1() {
        let input: Vec<&str> = INPUT.lines().collect();
        assert_eq!(solve1(&input), 13140)
    }

    #[test]
    fn test2() {
        let input: Vec<&str> = INPUT.lines().collect();

        let answer = [
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....\n",
        ]
        .join("\n");

        // println!("{}", solve2(&input));
        // println!("{answer}");
        assert_eq!(solve2(&input), answer)
    }
}
