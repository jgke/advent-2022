use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = Vec<((i32, i32), (i32, i32))>;

fn one_impl(input: &Input) -> i32 {
    let mut sum = 0;
    for ((f1, f2), (s1, s2)) in input {
        if (f1 <= s1 && f2 >= s2) || (f1 >= s1 && f2 <= s2) {
            sum += 1;
        }
    }
    sum
}

fn two_impl(input: &Input) -> i32 {
    let mut sum = 0;
    for ((f1, f2), (s1, s2)) in input {
        if !(f2 < s1 || f1 > s2) {
            sum += 1;
        }
    }
    sum
}

fn parse(reader: Vec<String>) -> Input {
    let mut lines = Vec::new();
    for s in reader {
        let p = s
            .split(',')
            .flat_map(|s| s.split('-').map(|s| s.parse().unwrap()))
            .collect::<Vec<_>>();
        lines.push(((p[0], p[1]), (p[2], p[3])));
    }
    lines
}

pub fn four() -> Result<(), std::io::Error> {
    let file = File::open("input/4_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 4 part 1: {}", one_impl(&input));
    println!("Day 4 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_4::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = [
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(2, one_impl(&parsed));
        assert_eq!(4, two_impl(&parsed));
    }
}
