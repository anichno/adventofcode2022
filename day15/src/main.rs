#[derive(Debug, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn dist(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    location: Coord,
    beacon: Coord,
}

impl Sensor {
    fn parse(input: &str) -> Self {
        let (_, num_start) = input.split_once('=').unwrap();
        let (sensor_x, input) = num_start.split_once(',').unwrap();
        let (_, num_start) = input.split_once('=').unwrap();
        let (sensor_y, input) = num_start.split_once(':').unwrap();
        let (_, num_start) = input.split_once('=').unwrap();
        let (beacon_x, input) = num_start.split_once(',').unwrap();
        let (_, beacon_y) = input.split_once('=').unwrap();

        let location = Coord {
            x: sensor_x.parse().unwrap(),
            y: sensor_y.parse().unwrap(),
        };
        let beacon = Coord {
            x: beacon_x.parse().unwrap(),
            y: beacon_y.parse().unwrap(),
        };

        Self { location, beacon }
    }
}

fn parse(input: &[&str]) -> Vec<Sensor> {
    input.iter().map(|l| Sensor::parse(l)).collect()
}

fn solve1(input: &[&str], row: isize) -> u32 {
    let sensors = parse(input);

    // find min and max x
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    for sensor in &sensors {
        min_x = min_x.min(sensor.location.x - sensor.location.dist(&sensor.beacon));
        max_x = max_x.max(sensor.location.x + sensor.location.dist(&sensor.beacon));
    }

    let mut num_invalid_beacon_pos = 0;
    'col: for x in min_x..=max_x {
        let test_pos = Coord { x, y: row };
        for sensor in &sensors {
            if test_pos.dist(&sensor.location) <= sensor.location.dist(&sensor.beacon)
                && test_pos != sensor.beacon
                && test_pos != sensor.location
            {
                num_invalid_beacon_pos += 1;
                continue 'col;
            }
        }
    }

    num_invalid_beacon_pos
}

fn solve2(input: &[&str], min_bound: isize, max_bound: isize) -> i64 {
    use z3::{
        ast::{Ast, Bool, Int},
        Config, Context,
    };
    let mut config = Config::new();
    config.set_model_generation(true);
    let context = Context::new(&config);
    let solver = z3::Solver::new(&context);

    let x = Int::new_const(&context, "x");
    let y = Int::new_const(&context, "y");

    solver.assert(&Bool::and(
        &context,
        &[
            &(x.ge(&Int::from_i64(&context, min_bound as i64))),
            &(x.le(&Int::from_i64(&context, max_bound as i64))),
        ],
    ));

    solver.assert(&Bool::and(
        &context,
        &[
            &(y.ge(&Int::from_i64(&context, min_bound as i64))),
            &(y.le(&Int::from_i64(&context, max_bound as i64))),
        ],
    ));

    let sensors = parse(input);

    let zero = Int::from_i64(&context, 0);

    for sensor in &sensors {
        let dist = Int::from_i64(&context, sensor.location.dist(&sensor.beacon) as i64);
        let sensor_x = Int::from_i64(&context, sensor.location.x as i64);
        let sensor_y = Int::from_i64(&context, sensor.location.y as i64);

        let x_diff = Int::fresh_const(&context, "x_diff");

        let x_clause = &(&x_diff._eq(&(&x - &sensor_x)) & &(&x - &sensor_x).ge(&zero))
            | &(&x_diff._eq(&(&sensor_x - &x)) & &(&x - &sensor_x).lt(&zero));

        solver.assert(&x_clause);

        let y_diff = Int::fresh_const(&context, "y_diff");
        let y_clause = &(&y_diff._eq(&(&y - &sensor_y)) & &(&y - &sensor_y).ge(&zero))
            | &(&y_diff._eq(&(&sensor_y - &y)) & &(&y - &sensor_y).lt(&zero));

        solver.assert(&y_clause);

        solver.assert(&(x_diff + y_diff).gt(&dist));
    }

    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let x = model.eval(&x, true).unwrap().as_i64().unwrap();
            let y = model.eval(&y, true).unwrap().as_i64().unwrap();
            x * 4_000_000 + y
        }
        _ => panic!("failed to sat"),
    }
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input, 2_000_000));
    println!("part 2: {}", solve2(&input, 0, 4_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT, 10), 26)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT, 0, 20), 56000011)
    }
}
