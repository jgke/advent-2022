use crate::grid::Grid;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = Vec<Vec<(i32, i32)>>;

fn one_impl(input: &Input) -> usize {
    let max_x = *input
        .iter()
        .flat_map(|row| row.iter().map(|(x, _)| x))
        .max()
        .unwrap()
        + 1;
    let max_y = *input
        .iter()
        .flat_map(|row| row.iter().map(|(_, y)| y))
        .max()
        .unwrap()
        + 1;

    let mut grid = Grid::new_with(max_x as usize, max_y as usize, |_, _| '.');
    for lineset in input {
        let mut start = lineset[0];
        for line in lineset {
            let dx = (line.0 - start.0).signum();
            let dy = (line.1 - start.1).signum();
            grid.set(start.0 as usize, start.1 as usize, '#');
            while *line != start {
                start.0 += dx;
                start.1 += dy;
                grid.set(start.0 as usize, start.1 as usize, '#');
            }
        }
    }

    let mut units = 0;
    'outer: loop {
        units += 1;
        let mut unit = (500, 0);
        loop {
            if grid.get(unit.0, unit.1 + 1).is_none() {
                break 'outer;
            }
            if grid.get(unit.0, unit.1 + 1) == Some(&'.') {
                unit = (unit.0, unit.1 + 1);
            } else if grid.get(unit.0 - 1, unit.1 + 1) == Some(&'.') {
                unit = (unit.0 - 1, unit.1 + 1);
            } else if grid.get(unit.0 + 1, unit.1 + 1) == Some(&'.') {
                unit = (unit.0 + 1, unit.1 + 1);
            } else {
                grid.set(unit.0, unit.1, '+');
                break;
            }
        }
    }

    units - 1
}

fn two_impl(input: &Input) -> usize {
    let max_y = *input
        .iter()
        .flat_map(|row| row.iter().map(|(_, y)| y))
        .max()
        .unwrap()
        + 2;
    let max_x = *input
        .iter()
        .flat_map(|row| row.iter().map(|(x, _)| x))
        .max()
        .unwrap()
        + max_y;

    let mut grid = Grid::new_with(max_x as usize, max_y as usize, |_, _| '.');
    for lineset in input {
        let mut start = lineset[0];
        for line in lineset {
            let dx = (line.0 - start.0).signum();
            let dy = (line.1 - start.1).signum();
            grid.set(start.0 as usize, start.1 as usize, '#');
            while *line != start {
                start.0 += dx;
                start.1 += dy;
                grid.set(start.0 as usize, start.1 as usize, '#');
            }
        }
    }

    let mut units = 0;
    loop {
        units += 1;
        let mut unit = (500, 0);
        if grid.get(unit.0, unit.1) == Some(&'+') {
            break;
        }
        loop {
            if grid.get(unit.0, unit.1 + 1) == Some(&'.') {
                unit = (unit.0, unit.1 + 1);
            } else if grid.get(unit.0 - 1, unit.1 + 1) == Some(&'.') {
                unit = (unit.0 - 1, unit.1 + 1);
            } else if grid.legal(unit.0 as i32 + 1, unit.1 as i32 + 1)
                && grid.get(unit.0 + 1, unit.1 + 1).unwrap_or(&'.') == &'.'
            {
                unit = (unit.0 + 1, unit.1 + 1);
            } else {
                grid.set(unit.0, unit.1, '+');
                break;
            }
        }
    }

    units - 1
}

fn parse(reader: Vec<String>) -> Input {
    reader
        .into_iter()
        .map(|row| {
            row.split(" -> ")
                .map(|pair| {
                    let split = pair.split(',').collect::<Vec<_>>();
                    (split[0].parse().unwrap(), split[1].parse().unwrap())
                })
                .collect()
        })
        .collect()
}

pub fn fourteen() -> Result<(), std::io::Error> {
    let file = File::open("input/14_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 14 part 1: {}", one_impl(&input));
    println!("Day 14 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_14::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(24, one_impl(&parsed));
        assert_eq!(93, two_impl(&parsed));
    }
}
