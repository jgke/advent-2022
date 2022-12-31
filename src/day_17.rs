use crate::grid::Grid;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = Vec<bool>;

fn check_delta(
    grid: &Grid<char>,
    rock: &[Vec<bool>],
    pos: (usize, usize),
    delta: (i32, i32),
) -> bool {
    for (y, row) in rock.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let pos = (
                (pos.0 as i32) + (x as i32) + delta.0,
                (pos.1 as i32) + (y as i32) + delta.1,
            );
            if c && (!grid.legal(pos.0, pos.1)
                || grid.get(pos.0 as usize, pos.1 as usize).unwrap_or(&'#') == &'#')
            {
                return true;
            }
        }
    }
    false
}

fn get_safe_bottom(grid: &Grid<char>, safe_y: usize) -> usize {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut best = safe_y;
    for x in 0..grid.row_size() {
        stack.push((x, safe_y));
    }
    while let Some((x, y)) = stack.pop() {
        if grid.get(x, y).unwrap_or(&'#') == &'#' {
            continue;
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        best = best.min(y);
        if x > 0 {
            stack.push((x - 1, y));
        }
        if x < grid.row_size() - 1 {
            stack.push((x + 1, y));
        }
        stack.push((x, y - 1));
    }

    best
}

fn simulate(input: &Input, count: usize) -> usize {
    let mut seen: HashMap<String, (usize, usize)> = HashMap::new();
    let mut movements = input.iter().cycle().copied();
    let mut grid = Grid::new_with(7, 1000, |_, _| '.');
    for x in 0..grid.row_size() {
        grid.set(x, 0, '#');
    }
    let rocks: Vec<Vec<Vec<bool>>> = vec![
        vec!["####"],
        vec![".#.", "###", ".#."],
        vec!["..#", "..#", "###"],
        vec!["#", "#", "#", "#"],
        vec!["##", "##"],
    ]
    .into_iter()
    .map(|rock| {
        rock.into_iter()
            .rev()
            .map(|row| row.chars().map(|c| c == '#').collect())
            .collect()
    })
    .collect();

    let mut collapsed_rows: usize = 0;
    let mut highest_point: usize = 1;
    let mut rock_index = 0;

    let mut i = 0;
    while i < count {
        i += 1;
        let mut rock_pos = (2, highest_point + 3);
        let rock = &rocks[rock_index];
        'outer: loop {
            let delta = if movements.next().unwrap() { 1 } else { -1 };

            if !check_delta(&grid, rock, rock_pos, (delta, 0)) {
                rock_pos = (((rock_pos.0 as i32) + delta) as usize, rock_pos.1);
            }

            if check_delta(&grid, rock, rock_pos, (0, -1)) {
                for (y, row) in rock.iter().enumerate() {
                    for (x, &c) in row.iter().enumerate() {
                        if c {
                            let pos = (rock_pos.0 + x, rock_pos.1 + y);
                            grid.set(pos.0, pos.1, '#');
                            highest_point = highest_point.max(pos.1 + 1);
                        }
                    }
                }
                break 'outer;
            }
            rock_pos = (rock_pos.0, rock_pos.1 - 1);
        }

        let bottom = get_safe_bottom(&grid, highest_point + 3) - 1;
        if bottom > 0 {
            collapsed_rows += bottom;
            highest_point -= bottom;

            grid = Grid::new_with(grid.row_size(), grid.col_size(), |x, y| {
                *grid.get(x, y + bottom).unwrap_or(&'.')
            });

            for x in 0..grid.row_size() {
                grid.set(x, 0, '#');
            }

            let mut grid_key = grid.iter().collect::<String>();
            for _ in 0..input.len() {
                grid_key.push(if movements.next().unwrap() { '>' } else { '<' });
            }
            if let Some((old_i, old_collapsed)) = seen.get(&grid_key) {
                let time_delta = i - old_i;
                let d = collapsed_rows - old_collapsed;
                let mut c = count / time_delta;
                if c > 2 {
                    c -= 2;

                    collapsed_rows += c * d;
                    i += c * time_delta;
                }
            } else {
                seen.insert(grid_key, (i, collapsed_rows));
            }
        }

        rock_index = (rock_index + 1) % rocks.len();
    }

    collapsed_rows + (highest_point - 1)
}

fn one_impl(input: &Input) -> usize {
    simulate(input, 2022)
}

fn two_impl(input: &Input) -> usize {
    simulate(input, 1000000000000)
}

fn parse(reader: Vec<String>) -> Input {
    reader[0]
        .chars()
        .map(|c| match c {
            '<' => false,
            '>' => true,
            other => unimplemented!("{}", other),
        })
        .collect()
}

pub fn seventeen() -> Result<(), std::io::Error> {
    let file = File::open("input/17_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 17 part 1: {}", one_impl(&input));
    println!("Day 17 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_17::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
            .trim()
            .lines()
            .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(3068, one_impl(&parsed));
        assert_eq!(1514285714288, two_impl(&parsed));
    }
}
