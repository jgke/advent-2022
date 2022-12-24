use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use scanf::sscanf;

type Input = (HashSet<((i32, i32), (i32, i32))>, HashSet<(i32, i32)>);

fn manhattan(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn one_impl(y: i32, input: &Input) -> usize {
    let mut covered = 0;
    let min_x = input
        .0
        .iter()
        .map(|(s, b)| s.0 - manhattan(*s, *b))
        .min()
        .unwrap();
    let max_x = input
        .0
        .iter()
        .map(|(s, b)| s.0 + manhattan(*s, *b))
        .max()
        .unwrap();

    for x in min_x..max_x {
        let mut c = '.';
        for (sensor, closest_beacon) in &input.0 {
            let d1 = manhattan(*sensor, *closest_beacon);
            let d2 = manhattan(*sensor, (x, y));
            if *sensor == (x, y) {
                c = 'S';
                break;
            } else if *closest_beacon == (x, y) {
                c = 'B';
                break;
            }
            if d2 <= d1 {
                c = '#';
            }
        }
        if c == '#' {
            covered += 1;
        }
    }

    covered
}

fn two_impl(max_x: i32, max_y: i32, input: &Input) -> usize {
    for (sensor, closest_beacon) in &input.0 {
        let d1 = manhattan(*sensor, *closest_beacon) + 1;
        let positions = (0..=d1)
            .flat_map(|d| {
                [
                    (sensor.0 + d, sensor.1 + (d1 - d)),
                    (sensor.0 - d, sensor.1 + (d1 - d)),
                    (sensor.0 + d, sensor.1 - (d1 - d)),
                    (sensor.0 - d, sensor.1 - (d1 - d)),
                ]
            })
            .collect::<HashSet<(i32, i32)>>();

        'outer: for (x, y) in positions {
            for (sensor, closest_beacon) in &input.0 {
                let d1 = manhattan(*sensor, *closest_beacon);
                let d2 = manhattan(*sensor, (x, y));
                if d2 <= d1 {
                    continue 'outer;
                }
            }
            if x >= 0 && x <= max_x && y >= 0 && y <= max_y {
                return x as usize * 4000000 + y as usize;
            }
        }
    }

    unreachable!()
}

fn parse(reader: Vec<String>) -> Input {
    let mut sensors = HashSet::new();
    let mut beacons = HashSet::new();
    for row in reader {
        let mut sens_x: i32 = 0;
        let mut sens_y: i32 = 0;
        let mut beac_x: i32 = 0;
        let mut beac_y: i32 = 0;
        sscanf!(
            &row,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            sens_x,
            sens_y,
            beac_x,
            beac_y
        )
        .unwrap();
        sensors.insert(((sens_x, sens_y), (beac_x, beac_y)));
        beacons.insert((beac_x, beac_y));
    }
    (sensors, beacons)
}

pub fn fifteen() -> Result<(), std::io::Error> {
    let file = File::open("15_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 15 part 1: {}", one_impl(2000000, &input));
    println!("Day 15 part 2: {}", two_impl(4000000, 4000000, &input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_15::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(26, one_impl(10, &parsed));
        assert_eq!(56000011, two_impl(20, 20, &parsed));
    }
}
