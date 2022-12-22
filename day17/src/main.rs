use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Jet> {
    let mut jets = Vec::new();
    for dir in input.chars() {
        jets.push(match dir {
            '>' => Jet::Right,
            '<' => Jet::Left,
            _ => panic!("Invalid jet dir: {dir}"),
        });
    }

    jets
}

#[derive(Default, Debug)]
struct Chute {
    columns: [Vec<bool>; 7],
}

impl Chute {
    fn new() -> Self {
        Default::default()
    }

    fn update_chute(&mut self, rock: FallingRock) {
        for part in rock.rock_parts {
            if part.y >= self.columns[part.x].len() {
                self.columns[part.x].resize(part.y + 1, false);
            }
            self.columns[part.x][part.y] = true;
        }
    }

    fn max_height(&self) -> usize {
        self.columns.iter().map(|col| col.len()).max().unwrap()
    }
}

impl Display for Chute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut outputs: [Vec<char>; 7] = Default::default();
        let max_height = self.max_height();
        for (x, col) in self.columns.iter().enumerate() {
            for val in col {
                if *val {
                    outputs[x].push('#');
                } else {
                    outputs[x].push('.');
                }
            }
            outputs[x].resize(max_height, '.');
            outputs[x] = outputs[x].iter().copied().rev().collect();
        }
        for y in 0..max_height {
            write!(f, "[")?;
            for x in 0..7 {
                write!(f, "{}", outputs[x][y])?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Shape {
    Dash,
    Plus,
    El,
    Line,
    Block,
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct FallingRock {
    rock_parts: Vec<Coord>,
    left_bound: usize,
    right_bound: usize,
    bottom_bound: usize,
}

impl FallingRock {
    fn spawn_rock(shape: Shape, height: usize) -> Self {
        let bottom_bound = height + 3;
        let left_bound = 2;
        let rock_parts: Vec<Coord> = match shape {
            Shape::Dash => (0..4)
                .into_iter()
                .map(|x| Coord {
                    x: left_bound + x,
                    y: bottom_bound,
                })
                .collect(),
            Shape::Plus => {
                let mut parts = vec![
                    Coord {
                        x: left_bound + 1,
                        y: bottom_bound + 2,
                    },
                    Coord {
                        x: left_bound + 1,
                        y: bottom_bound,
                    },
                ];
                parts.extend((0..3).into_iter().map(|x| Coord {
                    x: left_bound + x,
                    y: bottom_bound + 1,
                }));
                parts
            }
            Shape::El => vec![
                Coord {
                    x: left_bound + 2,
                    y: bottom_bound + 2,
                },
                Coord {
                    x: left_bound + 2,
                    y: bottom_bound + 1,
                },
                Coord {
                    x: left_bound + 0,
                    y: bottom_bound,
                },
                Coord {
                    x: left_bound + 1,
                    y: bottom_bound,
                },
                Coord {
                    x: left_bound + 2,
                    y: bottom_bound,
                },
            ],
            Shape::Line => (0..4)
                .into_iter()
                .map(|p| Coord {
                    x: left_bound + 0,
                    y: bottom_bound + p,
                })
                .collect(),
            Shape::Block => vec![
                Coord {
                    x: left_bound + 0,
                    y: bottom_bound,
                },
                Coord {
                    x: left_bound + 1,
                    y: bottom_bound,
                },
                Coord {
                    x: left_bound + 0,
                    y: bottom_bound + 1,
                },
                Coord {
                    x: left_bound + 1,
                    y: bottom_bound + 1,
                },
            ],
        };

        let right_bound = rock_parts.iter().map(|p| p.x).max().unwrap();
        Self {
            rock_parts,
            left_bound,
            right_bound,
            bottom_bound,
        }
    }

    fn shift_right(&mut self, chute: &Chute) {
        if self.right_bound < 6 {
            for part in self.rock_parts.iter() {
                if part.y < chute.columns[part.x + 1].len() && chute.columns[part.x + 1][part.y] {
                    return;
                }
            }
            for part in self.rock_parts.iter_mut() {
                part.x += 1;
            }
            self.left_bound += 1;
            self.right_bound += 1;
        }
    }

    fn shift_left(&mut self, chute: &Chute) {
        if self.left_bound > 0 {
            for part in self.rock_parts.iter() {
                if part.y < chute.columns[part.x - 1].len() && chute.columns[part.x - 1][part.y] {
                    return;
                }
            }
            for part in self.rock_parts.iter_mut() {
                part.x -= 1;
            }
            self.left_bound -= 1;
            self.right_bound -= 1;
        }
    }

    fn can_fall(&self, chute: &Chute) -> bool {
        for part in self.rock_parts.iter() {
            if part.y == 0 {
                return false;
            }
        }
        for (x, col) in chute.columns.iter().enumerate() {
            for part in self.rock_parts.iter().filter(|p| p.x == x) {
                if (part.y - 1) < col.len() && col[part.y - 1] {
                    return false;
                }
            }
        }
        true
    }

    fn fall(&mut self, chute: &Chute) -> bool {
        if self.can_fall(chute) {
            for part in self.rock_parts.iter_mut() {
                part.y -= 1;
            }
            self.bottom_bound -= 1;
            true
        } else {
            false
        }
    }
}

fn solve1(input: &str, num_rocks: usize) -> usize {
    let jets = parse_input(input);
    let shapes = &[
        Shape::Dash,
        Shape::Plus,
        Shape::El,
        Shape::Line,
        Shape::Block,
    ];

    let mut cur_jet = 0;
    let mut cur_rock_num = 0;
    let mut chute = Chute::new();

    while cur_rock_num < num_rocks {
        // spawn rock
        let mut rock =
            FallingRock::spawn_rock(shapes[cur_rock_num % shapes.len()], chute.max_height());

        // loop shift and fall
        loop {
            match jets[cur_jet % jets.len()] {
                Jet::Left => rock.shift_left(&chute),
                Jet::Right => rock.shift_right(&chute),
            }
            cur_jet += 1;
            if !rock.fall(&chute) {
                break;
            }
        }

        chute.update_chute(rock);
        cur_rock_num += 1;
    }

    chute.max_height()
}

fn solve2(input: &str, num_rocks: usize) -> usize {
    let jets = parse_input(input);
    let shapes = &[
        Shape::Dash,
        Shape::Plus,
        Shape::El,
        Shape::Line,
        Shape::Block,
    ];

    let mut cur_jet = 0;
    let mut cur_rock_num = 0;
    let mut chute = Chute::new();
    // let mut diffs = HashMap::new();
    // let mut diffs_prev = [None; 10000];
    // let mut diffs = [None; 10000];
    let mut height_diffs = Vec::new();
    let mut prev = 0;

    while cur_rock_num < 10000000 {
        height_diffs.push(chute.max_height() as i32 - prev);
        prev = chute.max_height() as i32;
        // if cur_rock_num % 10000 == 0 {
        //     println!(
        //         "{cur_rock_num}: {} -- {}",
        //         chute.max_height(),
        //         chute.max_height() - prev
        //     );
        //     prev = chute.max_height();
        // }
        // if cur_rock_num % shapes.len() == cur_jet % jets.len() {
        //     println!("{cur_rock_num}: jet {cur_jet} -- {}", chute.max_height());
        // }
        // spawn rock
        let mut rock =
            FallingRock::spawn_rock(shapes[cur_rock_num % shapes.len()], chute.max_height());

        // loop shift and fall
        loop {
            match jets[cur_jet % jets.len()] {
                Jet::Left => rock.shift_left(&chute),
                Jet::Right => rock.shift_right(&chute),
            }
            cur_jet += 1;
            if !rock.fall(&chute) {
                break;
            }
        }

        chute.update_chute(rock);
        cur_rock_num += 1;
        // if cur_rock_num % 1000 == 0 {
        //     println!("{}", chute.max_height());
        // }
    }

    for start_offset in 0..100000 {
        for loop_size in 5..10000 {
            // let diffs: Vec<i32> = height_diffs.iter().skip(start_offset).copied().collect();
            let diffs = &height_diffs[start_offset..];
            let mut windows = diffs.windows(loop_size);
            let (window1, window2) = (windows.next().unwrap(), windows.next().unwrap());
            if window1 == window2 {
                println!("{start_offset} -- {loop_size}");
            }
        }
    }

    // for start_idx in 0..1000 {
    //     for loop_interval in 1..1000 {
    //         let divisor = heights[start_idx + loop_interval];
    //         let next = heights[start_idx + (loop_interval * 2)];
    //         if next % divisor == 0 {
    //             if heights[start_idx + (loop_interval * 3)] % divisor == 0 {
    //                 println!("{start_idx} {loop_interval}");
    //             }
    //         }
    //     }
    // }
    // println!("{}", chute.max_height() * (1000000000000 / 1000));
    // // let jump = 12;
    // // let start = heights[12];
    // // for x in 1..100 {
    // //     if (start * (jump*x)) % 12 == 0;
    // // }
    // println! {"{}", heights[12]*(1000000000000/12)};

    // let diff = chute.max_height() as f64 / 100000_f64;
    // println!("{diff} {}", diff * 1000000000000_f64);

    chute.max_height()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", input.len());

    println!("part 1: {}", solve1(input, 2022));
    // println!("part 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT, 2022), 3068)
    }

    #[test]
    fn test1_b() {
        assert_eq!(solve1(include_str!("testinput.txt"), 2022), 3200)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT, 1000000000000), 1514285714288)
    }
}
