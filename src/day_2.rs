use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn new(p: char) -> Play {
        match p {
            'A' | 'X' => Play::Rock,
            'B' | 'Y' => Play::Paper,
            'C' | 'Z' => Play::Scissors,
            other => unimplemented!("{}", other),
        }
    }

    fn score(&self) -> i32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

fn one_impl(input: &[(Play, Play)]) -> i32 {
    input
        .iter()
        .map(|(theirs, ours)| match (theirs, ours) {
            (x, y) if x == y => 3 + ours.score(),
            (Play::Rock, Play::Paper) => 6 + ours.score(),
            (Play::Paper, Play::Scissors) => 6 + ours.score(),
            (Play::Scissors, Play::Rock) => 6 + ours.score(),
            _ => ours.score(),
        })
        .sum()
}

fn two_impl(input: &[(Play, Play)]) -> i32 {
    one_impl(
        &input
            .iter()
            .copied()
            .map(|(theirs, ours)| match (theirs, ours) {
                (Play::Rock, Play::Rock) => (theirs, Play::Scissors),
                (Play::Rock, Play::Paper) => (theirs, theirs),
                (Play::Rock, Play::Scissors) => (theirs, Play::Paper),

                (Play::Paper, Play::Rock) => (theirs, Play::Rock),
                (Play::Paper, Play::Paper) => (theirs, theirs),
                (Play::Paper, Play::Scissors) => (theirs, Play::Scissors),

                (Play::Scissors, Play::Rock) => (theirs, Play::Paper),
                (Play::Scissors, Play::Paper) => (theirs, theirs),
                (Play::Scissors, Play::Scissors) => (theirs, Play::Rock),
            })
            .collect::<Vec<_>>(),
    )
}

pub fn two() -> Result<(), std::io::Error> {
    let file = File::open("input/2_input")?;
    let reader = BufReader::new(file);
    let mut plays = vec![];
    for line in reader.lines() {
        let line = line?;
        plays.push((
            Play::new(line.chars().next().unwrap()),
            Play::new(line.chars().last().unwrap()),
        ));
    }
    println!("Day 2 part 1: {}", one_impl(&plays));
    println!("Day 2 part 2: {}", two_impl(&plays));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_2::{one_impl, two_impl, Play};

    #[test]
    fn it_works() {
        let lines = vec![
            (Play::Rock, Play::Paper),
            (Play::Paper, Play::Rock),
            (Play::Scissors, Play::Scissors),
        ];
        assert_eq!(15, one_impl(&lines));
        assert_eq!(12, two_impl(&lines));
    }
}
