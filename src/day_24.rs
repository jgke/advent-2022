use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn will_have_blizzard(
    time: usize,
    limits: (usize, usize),
    pos: (usize, usize),
    blizzards: &Blizzards,
) -> bool {
    let pos_l = (limits.0 + pos.0 - 1 - time % limits.0) % limits.0 + 1;
    let pos_r = (limits.0 + pos.0 - 1 + time % limits.0) % limits.0 + 1;
    let pos_t = (limits.1 + pos.1 - 1 - time % limits.1) % limits.1 + 1;
    let pos_b = (limits.1 + pos.1 - 1 + time % limits.1) % limits.1 + 1;
    blizzards.contains(&((pos_l, pos.1), '>'))
        || blizzards.contains(&((pos_r, pos.1), '<'))
        || blizzards.contains(&((pos.0, pos_t), 'v'))
        || blizzards.contains(&((pos.0, pos_b), '^'))
}

type Input = Vec<Vec<char>>;
type Blizzards = HashSet<((usize, usize), char)>;

fn solve(
    start: (usize, usize),
    end: (usize, usize),
    start_time: usize,
    limits: (usize, usize),
    blizzards: &Blizzards,
) -> usize {
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    positions.insert(start);

    let mut time = start_time;
    loop {
        let mut next_positions = HashSet::new();
        for (x, y) in positions {
            if (x, y) == end {
                return time + 1;
            }

            let mut try_move = |x, y| {
                if !will_have_blizzard(time + 1, limits, (x, y), blizzards) {
                    next_positions.insert((x, y));
                }
            };

            if y > 0 && x < limits.0 {
                try_move(x + 1, y);
            }
            if y > 0 && x > 1 {
                try_move(x - 1, y);
            }
            if y < limits.1 {
                try_move(x, y + 1);
            }
            if y > 1 {
                try_move(x, y - 1);
            }
            if y == 0 || !will_have_blizzard(time + 1, limits, (x, y), blizzards) {
                next_positions.insert((x, y));
            }
            if x == 1 && y == 1 {
                next_positions.insert((x, 0));
            }
            if x == limits.0 && y == limits.1 {
                next_positions.insert((x, y + 1));
            }
        }

        positions = next_positions;
        time += 1;
    }
}

fn one_impl(input: &Input) -> usize {
    let mut blizzards = HashSet::new();
    let start = (1, 0);
    let limits = (input[0].len() - 2, input.len() - 2);
    let end = (input[0].len() - 2, input.len() - 2);

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match *c {
                '>' | '<' | '^' | 'v' => {
                    blizzards.insert(((x, y), *c));
                }
                '.' | '#' => {}
                _ => unimplemented!(),
            }
        }
    }

    solve(start, end, 0, limits, &blizzards)
}

fn two_impl(input: &Input) -> usize {
    let mut blizzards = HashSet::new();
    let start = (1, 0);
    let limits = (input[0].len() - 2, input.len() - 2);
    let end = (input[0].len() - 2, input.len() - 2);

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match *c {
                '>' | '<' | '^' | 'v' => {
                    blizzards.insert(((x, y), *c));
                }
                '.' | '#' => {}
                _ => unimplemented!(),
            }
        }
    }

    solve(
        start,
        end,
        solve(
            end,
            start,
            solve(start, end, 0, limits, &blizzards),
            limits,
            &blizzards,
        ),
        limits,
        &blizzards,
    )
}

fn parse(reader: Vec<String>) -> Input {
    reader
        .into_iter()
        .map(|row| row.chars().collect())
        .collect()
}

pub fn twentyfour() -> Result<(), std::io::Error> {
    let file = File::open("24_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 24 part 1: {}", one_impl(&input));
    println!("Day 24 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_24::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(18, one_impl(&parsed));
        assert_eq!(54, two_impl(&parsed));
    }
}
