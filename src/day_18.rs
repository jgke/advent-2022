use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = Vec<(usize, usize, usize)>;

fn one_impl(input: &Input) -> usize {
    let minx = input.iter().map(|(x, _y, _z)| x).min().unwrap();
    let maxx = input.iter().map(|(x, _y, _z)| x).max().unwrap();
    let miny = input.iter().map(|(_x, y, _z)| y).min().unwrap();
    let maxy = input.iter().map(|(_x, y, _z)| y).max().unwrap();
    let minz = input.iter().map(|(_x, _y, z)| z).min().unwrap();
    let maxz = input.iter().map(|(_x, _y, z)| z).max().unwrap();

    let mut coords: Vec<Vec<Vec<bool>>> = (0..=maxz + 2)
        .map(|_| {
            (0..=maxy + 2)
                .map(|_| (0..=maxx + 2).map(|_| false).collect())
                .collect()
        })
        .collect();

    for (x, y, z) in input {
        coords[z + 1][y + 1][x + 1] = true;
    }

    let mut count = 0;

    for z in minz + 1..=maxz + 1 {
        for y in miny + 1..=maxy + 1 {
            for x in minx + 1..=maxx + 1 {
                if coords[z][y][x] {
                    if !coords[z - 1][y][x] {
                        count += 1
                    }
                    if !coords[z + 1][y][x] {
                        count += 1
                    }
                    if !coords[z][y - 1][x] {
                        count += 1
                    }
                    if !coords[z][y + 1][x] {
                        count += 1
                    }
                    if !coords[z][y][x - 1] {
                        count += 1
                    }
                    if !coords[z][y][x + 1] {
                        count += 1
                    }
                }
            }
        }
    }

    count
}

fn two_impl(input: &Input) -> usize {
    let maxx = input.iter().copied().map(|(x, _y, _z)| x).max().unwrap() + 4;
    let maxy = input.iter().copied().map(|(_x, y, _z)| y).max().unwrap() + 4;
    let maxz = input.iter().copied().map(|(_x, _y, z)| z).max().unwrap() + 4;

    let mut coords: Vec<Vec<Vec<bool>>> = (0..=maxz + 2)
        .map(|_| {
            (0..=maxy + 2)
                .map(|_| (0..=maxx + 2).map(|_| false).collect())
                .collect()
        })
        .collect();

    for (x, y, z) in input {
        coords[z + 1][y + 1][x + 1] = true;
    }

    let mut stack: Vec<(usize, usize, usize)> = Vec::new();
    let mut visited = HashSet::new();
    stack.push((0, 0, 0));

    let mut count = 0;

    while let Some((x, y, z)) = stack.pop() {
        if x >= maxx || y >= maxy || z >= maxz {
            continue;
        }
        if coords[z][y][x] {
            continue;
        }
        if visited.contains(&(x, y, z)) {
            continue;
        }
        visited.insert((x, y, z));
        if x > 0 && coords[z][y][x - 1] {
            count += 1;
        }
        if y > 0 && coords[z][y - 1][x] {
            count += 1;
        }
        if z > 0 && coords[z - 1][y][x] {
            count += 1;
        }
        if coords[z][y][x + 1] {
            count += 1;
        }
        if coords[z][y + 1][x] {
            count += 1;
        }
        if coords[z + 1][y][x] {
            count += 1;
        }

        if z > 0 {
            stack.push((x, y, z - 1));
        }
        if y > 0 {
            stack.push((x, y - 1, z));
        }
        if x > 0 {
            stack.push((x - 1, y, z));
        }
        stack.push((x + 1, y, z));
        stack.push((x, y + 1, z));
        stack.push((x, y, z + 1));
    }

    count
}

fn parse(reader: Vec<String>) -> Input {
    reader
        .iter()
        .map(|row| {
            let nums = row
                .split(',')
                .map(|t| t.parse().unwrap())
                .collect::<Vec<_>>();
            (nums[0], nums[1], nums[2])
        })
        .collect()
}

pub fn eighteen() -> Result<(), std::io::Error> {
    let file = File::open("input/18_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 18 part 1: {}", one_impl(&input));
    println!("Day 18 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_18::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(64, one_impl(&parsed));
        assert_eq!(58, two_impl(&parsed));
    }
}
