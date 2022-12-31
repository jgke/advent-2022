use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::{HashSet, VecDeque};

use crate::grid::Grid;

type Input = ((usize, usize), (usize, usize), Grid<i32>);

fn one_impl((start, end, grid): &Input) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((*start, 0));
    while let Some((pos, n)) = queue.pop_front() {
        if pos == *end {
            return n;
        }
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        let cur = *grid.get(pos.0, pos.1).unwrap();
        for (nx, ny) in grid.nbors(pos.0, pos.1) {
            if grid.get(nx, ny).unwrap() - cur <= 1 {
                queue.push_back(((nx, ny), n + 1));
            }
        }
    }
    unreachable!()
}

fn two_impl((_start, end, grid): &Input) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((*end, 0));
    while let Some((pos, n)) = queue.pop_front() {
        let cur = *grid.get(pos.0, pos.1).unwrap();
        if cur == 0 {
            return n;
        }
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        for (nx, ny) in grid.nbors(pos.0, pos.1) {
            if cur - grid.get(nx, ny).unwrap() <= 1 {
                queue.push_back(((nx, ny), n + 1));
            }
        }
    }
    unreachable!()
}

fn parse(reader: Vec<String>) -> Input {
    let chars: Vec<Vec<char>> = reader
        .into_iter()
        .map(|row| row.chars().collect())
        .collect();
    let mut start = None;
    let mut end = None;
    let grid = Grid::new_with(chars[0].len(), chars.len(), |x, y| {
        let mut c = chars[y][x];
        if c == 'S' {
            start = Some((x, y));
            c = 'a';
        }
        if c == 'E' {
            end = Some((x, y));
            c = 'z';
        }
        (c as i32) - ('a' as i32)
    });
    (start.unwrap(), end.unwrap(), grid)
}

pub fn twelve() -> Result<(), std::io::Error> {
    let file = File::open("input/12_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 12 part 1: {}", one_impl(&input));
    println!("Day 12 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_12::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(31, one_impl(&parsed));
        assert_eq!(29, two_impl(&parsed));
    }
}
