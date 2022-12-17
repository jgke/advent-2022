use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn one_impl(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|arr| arr.iter().sum()).max().unwrap()
}

fn two_impl(input: &[Vec<i32>]) -> i32 {
    let mut elfs = input.iter().map(|arr| arr.iter().sum()).collect::<Vec<_>>();
    elfs.sort();
    elfs.reverse();
    elfs.iter().take(3).sum()
}

pub fn one() -> Result<(), std::io::Error> {
    let file = File::open("1_input")?;
    let reader = BufReader::new(file);
    let mut elves = vec![];
    let mut elf = vec![];
    for line in reader.lines() {
        let line = line?;
        if line == "" {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.parse().unwrap());
        }
    }
    println!("Day 1 part 1: {}", one_impl(&elves));
    println!("Day 1 part 2: {}", two_impl(&elves));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_1::{one_impl, two_impl};

    #[test]
    fn it_works() {
        let lines = vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000, 8000, 9000], vec![10000]];
        assert_eq!(24000, one_impl(&lines));
        assert_eq!(45000, two_impl(&lines));

    }
}
