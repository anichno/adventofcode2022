use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

struct Map {
    grid: Vec<Vec<u8>>,
    start: Coord,
    end: Coord,
}

fn letter_to_height(letter: u8) -> u8 {
    assert!((97..=122).contains(&letter));
    letter - 97
}

fn parse_input(input: &[&str]) -> Map {
    let mut grid = Vec::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in input.iter().enumerate() {
        let mut row = Vec::new();
        for (x, col) in line.as_bytes().iter().enumerate() {
            if *col == b'S' {
                start = Some(Coord { x, y });
                row.push(0);
            } else if *col == b'E' {
                end = Some(Coord { x, y });
                row.push(letter_to_height(b'z'));
            } else {
                row.push(letter_to_height(*col));
            }
        }
        grid.push(row);
    }

    let (Some(start), Some(end)) = (start, end) else {
        panic!();
    };

    Map { grid, start, end }
}

fn solve1(input: &[&str]) -> usize {
    let map = parse_input(input);

    let mut dist = vec![vec![usize::MAX; map.grid[0].len()]; map.grid.len()];
    let mut queue = VecDeque::new();
    for y in 0..map.grid.len() {
        for x in 0..map.grid[0].len() {
            queue.push_back(Coord { x, y });
        }
    }
    dist[map.start.y][map.start.x] = 0;

    while !queue.is_empty() {
        queue
            .make_contiguous()
            .sort_unstable_by_key(|i| dist[i.y][i.x]);
        let cur_pos = queue.pop_front().unwrap();
        if cur_pos == map.end {
            break;
        }
        let dist_to_cur = dist[cur_pos.y][cur_pos.x];

        let cur_height = map.grid[cur_pos.y][cur_pos.x];

        for neighbor in [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(x, y)| (x + cur_pos.x as isize, y + cur_pos.y as isize))
            .filter(|(x, y)| {
                *x >= 0
                    && *y >= 0
                    && *x < map.grid[0].len() as isize
                    && *y < map.grid.len() as isize
            })
            .map(|(x, y)| Coord {
                x: x as usize,
                y: y as usize,
            })
            .filter(|c| {
                map.grid[c.y][c.x] <= cur_height + 1 && queue.iter().any(|coord| *coord == *c)
            })
        {
            let prev_dist = dist[neighbor.y][neighbor.x];

            let new_dist = dist_to_cur + 1;
            if new_dist < prev_dist {
                dist[neighbor.y][neighbor.x] = new_dist;
            }
        }
    }

    dist[map.end.y][map.end.x]
}

fn solve2(input: &[&str]) -> usize {
    let map = parse_input(input);

    let mut end_positions = HashSet::new();
    for (y, row) in map.grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 0 {
                end_positions.insert(Coord { x, y });
            }
        }
    }
    let mut shortest_path = usize::MAX;

    let mut dist = vec![vec![usize::MAX; map.grid[0].len()]; map.grid.len()];
    let mut queue = VecDeque::new();
    for y in 0..map.grid.len() {
        for x in 0..map.grid[0].len() {
            queue.push_back(Coord { x, y });
        }
    }
    dist[map.end.y][map.end.x] = 0;

    while !queue.is_empty() && !end_positions.is_empty() {
        queue
            .make_contiguous()
            .sort_unstable_by_key(|i| dist[i.y][i.x]);
        let cur_pos = queue.pop_front().unwrap();

        if end_positions.contains(&cur_pos) {
            shortest_path = shortest_path.min(dist[cur_pos.y][cur_pos.x]);
            end_positions.remove(&cur_pos);
            continue;
        }

        let dist_to_cur = dist[cur_pos.y][cur_pos.x];
        let cur_height = map.grid[cur_pos.y][cur_pos.x];

        for neighbor in [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(x, y)| (x + cur_pos.x as isize, y + cur_pos.y as isize))
            .filter(|(x, y)| {
                *x >= 0
                    && *y >= 0
                    && *x < map.grid[0].len() as isize
                    && *y < map.grid.len() as isize
            })
            .map(|(x, y)| Coord {
                x: x as usize,
                y: y as usize,
            })
            .filter(|c| {
                map.grid[c.y][c.x] >= cur_height.saturating_sub(1)
                    && queue.iter().any(|coord| *coord == *c)
            })
        {
            let prev_dist = dist[neighbor.y][neighbor.x];

            let new_dist = dist_to_cur + 1;
            if new_dist < prev_dist {
                dist[neighbor.y][neighbor.x] = new_dist;
            }
        }
    }

    shortest_path
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 31)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 29)
    }
}
