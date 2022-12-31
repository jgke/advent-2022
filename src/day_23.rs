use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = Vec<Vec<char>>;

fn one_impl(input: &Input) -> (i32, usize) {
    let mut elves: HashSet<(i32, i32)> = HashSet::new();
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }
    let elf_count = elves.len();
    let mut movelist = [0, 1, 2, 3];

    let mut score_on_ten = 0;

    for i in 0..usize::MAX {
        assert_eq!(elf_count, elves.len());
        let mut wanted_targets: HashMap<(i32, i32), i32> = HashMap::new();
        let mut moves: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

        for &(x, y) in &elves {
            moves.insert((x, y), (x, y));
            {
                let elves = &elves;
                if !(-1..=1)
                    .into_iter()
                    .flat_map(|dy| {
                        (-1..=1).into_iter().filter_map(move |dx| {
                            if dx != 0 || dy != 0 {
                                Some((x + dx, y + dy))
                            } else {
                                None
                            }
                        })
                    })
                    .any(|(x, y)| elves.contains(&(x, y)))
                {
                    continue;
                }
            }
            for m in movelist {
                match m {
                    0 => {
                        if !elves.contains(&(x - 1, y - 1))
                            && !elves.contains(&(x, y - 1))
                            && !elves.contains(&(x + 1, y - 1))
                        {
                            *wanted_targets.entry((x, y - 1)).or_insert(0) += 1;
                            moves.insert((x, y), (x, y - 1));
                            break;
                        }
                    }
                    1 => {
                        if !elves.contains(&(x - 1, y + 1))
                            && !elves.contains(&(x, y + 1))
                            && !elves.contains(&(x + 1, y + 1))
                        {
                            *wanted_targets.entry((x, y + 1)).or_insert(0) += 1;
                            moves.insert((x, y), (x, y + 1));
                            break;
                        }
                    }
                    2 => {
                        if !elves.contains(&(x - 1, y - 1))
                            && !elves.contains(&(x - 1, y))
                            && !elves.contains(&(x - 1, y + 1))
                        {
                            *wanted_targets.entry((x - 1, y)).or_insert(0) += 1;
                            moves.insert((x, y), (x - 1, y));
                            break;
                        }
                    }
                    3 => {
                        if !elves.contains(&(x + 1, y - 1))
                            && !elves.contains(&(x + 1, y))
                            && !elves.contains(&(x + 1, y + 1))
                        {
                            *wanted_targets.entry((x + 1, y)).or_insert(0) += 1;
                            moves.insert((x, y), (x + 1, y));
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        let mut new_elves = HashSet::new();

        let mut any_moved = false;
        for (from, to) in moves {
            if from != to && wanted_targets[&to] == 1 {
                any_moved = true;
                new_elves.insert(to);
            } else {
                new_elves.insert(from);
            }
        }
        if !any_moved {
            return (score_on_ten, i + 1);
        }

        elves = new_elves;

        movelist = [movelist[1], movelist[2], movelist[3], movelist[0]];

        if i == 9 {
            let mut minx = i32::MAX;
            let mut maxx = i32::MIN;
            let mut miny = i32::MAX;
            let mut maxy = i32::MIN;

            for &(x, y) in &elves {
                minx = minx.min(x);
                maxx = maxx.max(x);
                miny = miny.min(y);
                maxy = maxy.max(y);
            }

            score_on_ten = (maxx - minx + 1) * (maxy - miny + 1) - (elves.len() as i32);
        }
    }

    unreachable!()
}

fn parse(reader: Vec<String>) -> Input {
    reader
        .into_iter()
        .map(|row| row.chars().collect())
        .collect()
}

pub fn twentythree() -> Result<(), std::io::Error> {
    let file = File::open("input/23_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 23 part 1&2: {:?}", one_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_23::{one_impl, parse};

    #[test]
    fn it_works() {
        let input = "
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!((110, 20), one_impl(&parsed));
    }
}
