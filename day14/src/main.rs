use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum Thing {
    Rock,
    Sand,
    Empty,
}

#[derive(Debug)]
struct Cave {
    grid: Vec<Thing>,
    x_min: usize,
    width: usize,
    height: usize,
}

impl Cave {
    fn parse<T: AsRef<str>>(input: &[T]) -> Self {
        let mut x_min = usize::MAX;
        let mut x_max = 0;
        let mut max_y = 0;

        // find x_offset and max_y
        for line in input {
            let line = line.as_ref();
            line.split(" -> ")
                .map(|s| s.split_once(',').unwrap())
                .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                .for_each(|(l, r)| {
                    x_min = x_min.min(l);
                    x_max = x_max.max(l);
                    max_y = max_y.max(r)
                });
        }

        let width = x_max - x_min + 1;
        max_y += 1;
        let mut grid = vec![Thing::Empty; (max_y) * width];

        for line in input {
            let line = line.as_ref();
            let mut rock_coords = line
                .split(" -> ")
                .map(|s| s.split_once(',').unwrap())
                .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()));

            let mut start = rock_coords.next().unwrap();
            for tgt in rock_coords {
                let start_x = start.0.min(tgt.0);
                let end_x = start.0.max(tgt.0);
                let start_y = start.1.min(tgt.1);
                let end_y = start.1.max(tgt.1);

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        grid[y * width + (x - x_min)] = Thing::Rock;
                    }
                }

                start = tgt;
            }
        }

        Self {
            grid,
            x_min,
            width,
            height: max_y,
        }
    }

    fn coord_in_bounds(&self, x: usize, y: usize) -> bool {
        if x.checked_sub(self.x_min).is_none() {
            return false;
        }
        (0..self.width).contains(&(x - self.x_min)) && (0..self.height).contains(&y)
    }

    fn get(&self, x: usize, y: usize) -> &Thing {
        assert!(self.coord_in_bounds(x, y));
        &self.grid[y * self.width + (x - self.x_min)]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Thing {
        assert!(self.coord_in_bounds(x, y));
        &mut self.grid[y * self.width + (x - self.x_min)]
    }

    fn add_sand(&mut self) -> bool {
        let sand_start_x = 500;
        if let Thing::Sand = self.get(sand_start_x, 0) {
            return false;
        }

        let mut sand_x = sand_start_x;
        let mut sand_y = 0;
        loop {
            let new_pos = [
                (sand_x, sand_y + 1),
                (sand_x - 1, sand_y + 1),
                (sand_x + 1, sand_y + 1),
            ]
            .iter()
            .filter(|(x, y)| {
                self.coord_in_bounds(*x, *y) && matches!(self.get(*x, *y), &Thing::Empty)
            })
            .copied()
            .next();

            if let Some(new_pos) = new_pos {
                sand_x = new_pos.0;
                sand_y = new_pos.1;
            } else {
                break;
            }
        }

        if [
            (sand_x, sand_y + 1),
            (sand_x - 1, sand_y + 1),
            (sand_x + 1, sand_y + 1),
        ]
        .iter()
        .any(|(x, y)| !self.coord_in_bounds(*x, *y))
        {
            false
        } else {
            *self.get_mut(sand_x, sand_y) = Thing::Sand;
            true
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in self.x_min..(self.width + self.x_min) {
                let draw = match self.get(x, y) {
                    Thing::Rock => "#",
                    Thing::Sand => "o",
                    Thing::Empty => ".",
                };
                write!(f, "{draw}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn solve1(input: &[&str]) -> usize {
    let mut cave = Cave::parse(input);

    let mut num_sand = 0;
    while cave.add_sand() {
        num_sand += 1;
    }
    num_sand
}

/// This is jank AF
fn solve2(input: &[&str]) -> u32 {
    let cave = Cave::parse(input);
    let floor_height = cave.height + 1;
    let x_min = cave.x_min - 200;
    let x_max = x_min + cave.width + 200 + 200;

    let mut input: Vec<String> = input.iter().map(|s| s.to_string()).collect();
    input.push(format!("{x_min},{floor_height} -> {x_max},{floor_height}"));

    let mut cave = Cave::parse(&input);

    let mut num_sand = 0;
    while cave.add_sand() {
        num_sand += 1;
    }
    num_sand
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
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 24)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 93)
    }
}
