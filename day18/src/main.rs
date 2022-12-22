use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn adjacent(&self, other: &Self) -> bool {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs() == 1
    }
}

fn parse_input(input: &[&str]) -> Vec<Point> {
    let mut points = Vec::new();
    for line in input {
        let mut parts = line.split(',');
        let (Some(x), Some(y), Some(z)) = (parts.next(), parts.next(), parts.next()) else {
            panic!()
        };
        let (x, y, z) = (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
        points.push(Point { x, y, z });
    }

    points
}

fn solve1(input: &[&str]) -> usize {
    let points = parse_input(input);
    let mut sides_exposed = HashMap::new();

    for point1 in points.iter() {
        if !sides_exposed.contains_key(point1) {
            sides_exposed.insert(point1, 6);
        }
        for point2 in points.iter() {
            if point1.adjacent(point2) {
                *sides_exposed.get_mut(point1).unwrap() -= 1;
            }
        }
    }

    sides_exposed.values().sum()
}

fn solve2(input: &[&str]) -> u32 {
    fn flood_fill_air(
        cur_pos: Point,
        grid: &Vec<Vec<Vec<bool>>>,
        air: &mut Vec<Vec<Vec<bool>>>,
    ) -> u32 {
        let mut droplet_walls = 0;
        for diff in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let new_x: Result<usize, _> = (cur_pos.x + diff.0).try_into();
            let new_y: Result<usize, _> = (cur_pos.y + diff.1).try_into();
            let new_z: Result<usize, _> = (cur_pos.z + diff.2).try_into();

            if let (Ok(new_x), Ok(new_y), Ok(new_z)) = (new_x, new_y, new_z) {
                if new_x < grid.len() && new_y < grid[0].len() && new_z < grid[0][0].len() {
                    if grid[new_x][new_y][new_z] {
                        droplet_walls += 1;
                    } else if !air[new_x][new_y][new_z] {
                        //flood!
                        air[new_x][new_y][new_z] = true;
                        droplet_walls += flood_fill_air(
                            Point {
                                x: new_x as isize,
                                y: new_y as isize,
                                z: new_z as isize,
                            },
                            grid,
                            air,
                        );
                    }
                }
            }
        }

        droplet_walls
    }

    let mut points = parse_input(input);

    // bump everything by 1
    for point in points.iter_mut() {
        point.x += 1;
        point.y += 1;
        point.z += 1;
    }

    let (mut max_x, mut max_y, mut max_z) = (isize::MIN, isize::MIN, isize::MIN);

    for p in &points {
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
        max_z = max_z.max(p.z);
    }

    let mut grid =
        vec![vec![vec![false; max_z as usize + 2]; max_y as usize + 2]; max_x as usize + 2];
    for p in &points {
        grid[p.x as usize][p.y as usize][p.z as usize] = true;
    }

    let mut air = grid.clone();
    flood_fill_air(Point { x: 0, y: 0, z: 0 }, &grid, &mut air)
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
        "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6", "1,2,5",
        "3,2,5", "2,1,5", "2,3,5",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 64)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 58)
    }
}