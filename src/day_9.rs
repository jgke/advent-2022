use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Input = Vec<(Direction, usize)>;

fn move_head(head: (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (head.0, head.1 + 1),
        Direction::Down => (head.0, head.1 - 1),
        Direction::Left => (head.0 - 1, head.1),
        Direction::Right => (head.0 + 1, head.1),
    }
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let table = vec![
        vec![(-1, -1), (-1, -1), (-0, -1), (1, -1), (1, -1)],
        vec![(-1, -1), (-0, -0), (-0, -0), (0, -0), (1, -1)],
        vec![(-1, -0), (-0, -0), (-0, -0), (0, -0), (1, -0)],
        vec![(-1, 1), (-0, 0), (-0, 0), (0, 0), (1, 1)],
        vec![(-1, 1), (-1, 1), (-0, 1), (1, 1), (1, 1)],
    ];
    let d = table[(head.1 - tail.1 + 2) as usize][(head.0 - tail.0 + 2) as usize];
    (tail.0 + d.0, tail.1 + d.1)
}

fn one_impl(input: &Input) -> usize {
    let mut history: HashSet<(i32, i32)> = HashSet::new();
    let mut tail = (0, 0);
    let mut head = (0, 0);
    history.insert(tail);
    for (dir, count) in input {
        for _ in 0..*count {
            head = move_head(head, *dir);
            tail = move_tail(head, tail);
            history.insert(tail);
        }
    }
    history.len()
}

fn two_impl(input: &Input) -> usize {
    let mut history: HashSet<(i32, i32)> = HashSet::new();
    let mut chain: Vec<(i32, i32)> = (0..10).map(|_| (0, 0)).collect();
    history.insert((0, 0));
    for (dir, count) in input {
        for _ in 0..*count {
            chain[0] = move_head(chain[0], *dir);
            for i in 1..chain.len() {
                chain[i] = move_tail(chain[i - 1], chain[i]);
            }
            history.insert(*chain.last().unwrap());
        }
    }
    history.len()
}

fn parse(reader: Vec<String>) -> Input {
    reader
        .into_iter()
        .map(|movement| {
            let split = movement.split(' ').collect::<Vec<_>>();
            (
                match split[0] {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    other => unimplemented!("{}", other),
                },
                split[1].parse().unwrap(),
            )
        })
        .collect()
}

pub fn nine() -> Result<(), std::io::Error> {
    let file = File::open("9_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 9 part 1: {}", one_impl(&input));
    println!("Day 9 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_9::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(13, one_impl(&parsed));
        assert_eq!(1, two_impl(&parsed));
    }

    #[test]
    fn it_works_2() {
        let input = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(36, two_impl(&parsed));
    }
}
