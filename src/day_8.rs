use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::grid::Grid;

type Input = Grid<i32>;

fn ray(trees: &Input, target: &mut Grid<bool>, (mut x, mut y): (i32, i32), (dx, dy): (i32, i32)) {
    let mut max = i32::MIN;
    while let Some(tree) = trees.get(x as usize, y as usize) {
        if *tree > max {
            max = *tree;
            target.set(x as usize, y as usize, true);
        }
        x += dx;
        y += dy;
    }
}

fn ray2(trees: &Input, (mut x, mut y): (i32, i32), (dx, dy): (i32, i32)) -> i32 {
    let max = trees.get(x as usize, y as usize).unwrap();
    x += dx;
    y += dy;
    let mut count = 0;
    while let Some(tree) = trees.get(x as usize, y as usize) {
        count += 1;
        if tree >= max {
            break;
        }
        x += dx;
        y += dy;
    }
    dbg!(count)
}

fn one_impl(input: &Input) -> usize {
    let mut grid = input.map(|_| false);
    for y in 0..input.col_size() {
        ray(input, &mut grid, (0, y as i32), (1, 0));
        ray(
            input,
            &mut grid,
            (input.row_size() as i32 - 1, y as i32),
            (-1, 0),
        );
    }

    for x in 0..input.row_size() {
        ray(input, &mut grid, (x as i32, 0), (0, 1));
        ray(
            input,
            &mut grid,
            (x as i32, input.col_size() as i32 - 1),
            (0, -1),
        );
    }

    grid.iter().map(|c| *c as usize).sum()
}

fn two_impl(input: &Input) -> i32 {
    let mut max = 0;
    for y in 1..input.col_size() - 1 {
        for x in 1..input.row_size() - 1 {
            let mut res = 1;

            res *= ray2(input, (x as i32, y as i32), (1, 0));
            res *= ray2(input, (x as i32, y as i32), (-1, 0));
            res *= ray2(input, (x as i32, y as i32), (0, 1));
            res *= ray2(input, (x as i32, y as i32), (0, -1));

            max = max.max(res);
            println!("x={} y={} {}\n", x, y, res);
        }
    }
    max
}

fn parse(reader: Vec<String>) -> Input {
    Grid::new(
        reader
            .into_iter()
            .map(|c| c.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
            .collect(),
    )
}

pub fn eight() -> Result<(), std::io::Error> {
    let file = File::open("8_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 8 part 1: {}", one_impl(&input));
    println!("Day 8 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_8::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = ["30373", "25512", "65332", "33549", "35390"];
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(21, one_impl(&parsed));
        assert_eq!(8, two_impl(&parsed));
    }
}
