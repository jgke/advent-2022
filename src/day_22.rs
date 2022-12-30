use crate::grid::Grid;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug, Copy)]
enum Movement {
    Rotate { right: bool },
    Move(i32),
}

type Input = (Grid<char>, Vec<Movement>);

fn get_rotation(rotation: usize) -> (i32, i32) {
    match rotation {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        3 => (0, -1),
        _ => unreachable!(),
    }
}

fn one_impl((grid, movements): &Input) -> usize {
    let mut pos = (0, 1);
    let mut rotation = 0;

    for x in 0..grid.row_size() {
        if grid.get(x, 1).unwrap() == &'.' {
            pos = (x, 1);
            break;
        }
    }

    for movement in movements {
        match *movement {
            Movement::Rotate { right } => {
                if right {
                    rotation = (rotation + 1) % 4;
                } else {
                    rotation = (rotation + 3) % 4;
                }
            }
            Movement::Move(n) => {
                let (x, y) = get_rotation(rotation);

                for _ in 0..n {
                    let mut npos: (usize, usize) = (
                        (pos.0 as i32 + x).rem_euclid(grid.row_size() as i32) as usize,
                        (pos.1 as i32 + y).rem_euclid(grid.col_size() as i32) as usize,
                    );

                    while *grid.get(npos.0, npos.1).unwrap_or(&' ') != '.'
                        && *grid.get(npos.0, npos.1).unwrap_or(&' ') != '#'
                    {
                        npos = (
                            (npos.0 as i32 + x).rem_euclid(grid.row_size() as i32) as usize,
                            (npos.1 as i32 + y).rem_euclid(grid.col_size() as i32) as usize,
                        );
                    }

                    match *grid.get(npos.0, npos.1).unwrap_or(&' ') {
                        '#' => break,
                        '.' => pos = npos,
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    pos.1 * 1000 + pos.0 * 4 + rotation
}

fn two_impl((grid, movements): &Input) -> usize {
    let mut pos = (0, 1);
    let mut rotation = 0;

    let translations = get_translations(grid);

    for x in 0..grid.row_size() {
        if grid.get(x, 1).unwrap() == &'.' {
            pos = (x, 1);
            break;
        }
    }

    for movement in movements {
        match *movement {
            Movement::Rotate { right } => {
                if right {
                    rotation = (rotation + 1) % 4;
                } else {
                    rotation = (rotation + 3) % 4;
                }
            }
            Movement::Move(n) => {
                let (mut x, mut y) = get_rotation(rotation);

                for _ in 0..n {
                    let mut npos: (usize, usize) = (
                        (pos.0 as i32 + x).rem_euclid(grid.row_size() as i32) as usize,
                        (pos.1 as i32 + y).rem_euclid(grid.col_size() as i32) as usize,
                    );

                    if grid
                        .get(npos.0, npos.1)
                        .unwrap_or(&' ')
                        .is_ascii_alphabetic()
                    {
                        let new_npos = translations[&npos];
                        let new_rotation = match dbg!(
                            grid.get(new_npos.0 + 1, new_npos.1),
                            grid.get(new_npos.0, new_npos.1 + 1),
                            grid.get(new_npos.0.saturating_sub(1), new_npos.1),
                            grid.get(new_npos.0, new_npos.1.saturating_sub(1)),
                        ) {
                            (Some(&'.' | &'#'), _, _, _) if npos != new_npos || x <= 0 => 0,
                            (_, Some(&'.' | &'#'), _, _) if npos != new_npos || y <= 0 => 1,
                            (_, _, Some(&'.' | &'#'), _) if npos != new_npos || x >= 0 => 2,
                            (_, _, _, Some(&'.' | &'#')) if npos != new_npos || y >= 0 => 3,
                            (_, _, _, _) => unreachable!(),
                        };
                        (x, y) = get_rotation(new_rotation);
                        npos = (
                            (new_npos.0 as i32 + x).rem_euclid(grid.row_size() as i32) as usize,
                            (new_npos.1 as i32 + y).rem_euclid(grid.col_size() as i32) as usize,
                        );
                        assert!(!grid.get(npos.0, npos.1).unwrap().is_ascii_alphabetic());
                        if grid.get(npos.0, npos.1).unwrap() == &'#' {
                            break;
                        }
                        rotation = new_rotation;
                    }

                    match *grid.get(npos.0, npos.1).unwrap_or(&' ') {
                        '#' => break,
                        '.' => pos = npos,
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    pos.1 * 1000 + pos.0 * 4 + rotation
}

fn get_translations(grid: &Grid<char>) -> HashMap<(usize, usize), (usize, usize)> {
    let mut stacks: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut stacks_done: HashSet<char> = HashSet::new();

    let mut out = HashMap::new();

    grid.map_pos(|c, x, y| {
        if c.is_ascii_lowercase() {
            out.insert((x, y), (x, y));
        }
    });

    let mut start = (0, 0);

    for x in 0..grid.row_size() {
        if grid.get(x, 0).unwrap() == &'z' {
            start = (x, 0);
            break;
        }
    }

    let mut prev = (start.0, start.1);
    let mut cur = (start.0, start.1 + 1);
    let mut prev_c = 'z';

    while cur != start {
        let cur_c = *grid.get(cur.0, cur.1).unwrap();
        assert!(cur_c.is_ascii_alphabetic());

        if cur_c != prev_c {
            stacks_done.insert(prev_c);
        }

        if cur_c.is_ascii_uppercase() && stacks_done.contains(&cur_c) {
            let other = stacks.get_mut(&cur_c).unwrap().pop().unwrap();
            out.insert(cur, other);
            out.insert(other, cur);
        } else if cur_c.is_ascii_uppercase() {
            stacks.entry(cur_c).or_insert_with(Vec::new).push(cur);
        }

        let next = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .copied()
            .map(|(dx, dy)| ((cur.0 as i32) + dx, (cur.1 as i32) + dy))
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|pos| prev != *pos)
            .find(|pos| {
                grid.get(pos.0, pos.1)
                    .filter(|c| c.is_ascii_alphabetic())
                    .is_some()
            })
            .unwrap();
        prev_c = cur_c;
        prev = cur;
        cur = next;
    }

    out
}

fn parse(mut reader: Vec<String>) -> Input {
    let last_row = reader.pop().unwrap().chars().collect::<Vec<_>>();
    reader.pop();
    let grid_chars: Vec<Vec<char>> = reader.iter().map(|row| row.chars().collect()).collect();
    let grid = Grid::new_with(
        reader.iter().map(|r| r.len()).max().unwrap(),
        grid_chars.len(),
        |x, y| *grid_chars[y].get(x).unwrap_or(&' '),
    );
    let mut movements = Vec::new();
    let mut i = 0;
    while i < last_row.len() {
        i += 1;
        let mut s = String::new();
        match last_row[i - 1] {
            'L' | 'R' => {
                movements.push(Movement::Rotate {
                    right: last_row[i - 1] == 'R',
                });
                continue;
            }
            c => s.push(c),
        }
        while last_row.get(i).unwrap_or(&' ').is_ascii_digit() {
            s.push(last_row[i]);
            i += 1;
        }
        movements.push(Movement::Move(s.parse().unwrap()));
    }
    (grid, movements)
}

pub fn twentytwo() -> Result<(), std::io::Error> {
    let file = File::open("22_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 22 part 1: {}", one_impl(&input));
    println!("Day 22 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_22::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "        zAAAAz
        B...#F
        B.#..F
        B#...F
zAAAABBBb....F
C...#.......#G
C........#...G
C..#....#....G
C..........#.gGGGz
zDDDDEEEe...#....F
        E.....#..F
        E.#......F
        E......#.F
        zDDDDCCCCz

10R5L5R10L4R5L5"
            .lines()
            .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(6032, one_impl(&parsed));
        assert_eq!(5031, two_impl(&parsed));
    }
}
