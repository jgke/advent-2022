use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = (Vec<i64>, i64);

fn one_impl(input: &Input) -> i64 {
    let mut nums = input.0.iter().copied().enumerate().collect::<Vec<_>>();

    for x in 0..nums.len() {
        for i in 0..nums.len() {
            if nums[i].0 == x {
                let (_, n) = nums[i];
                let mut new_pos = (((i as i64) + n).rem_euclid(input.0.len() as i64 - 1)) as usize;
                if new_pos == 0 {
                    new_pos = input.0.len() - 1;
                }

                nums.remove(i);
                nums.insert(new_pos, (x, n));
                break;
            }
        }
    }

    let cycled = nums
        .iter()
        .map(|(_, n)| n)
        .cycle()
        .copied()
        .skip_while(|n| *n != 0)
        .take(3001)
        .collect::<Vec<_>>();
    cycled[1000] + cycled[2000] + cycled[3000]
}

fn two_impl(input: &Input) -> i64 {
    let mut nums = input
        .0
        .iter()
        .copied()
        .map(|n| n * input.1)
        .enumerate()
        .collect::<Vec<_>>();

    for _ in 0..10 {
        for x in 0..nums.len() {
            for i in 0..nums.len() {
                if nums[i].0 == x {
                    let (_, n) = nums[i];
                    let mut new_pos =
                        (((i as i64) + n).rem_euclid(input.0.len() as i64 - 1)) as usize;
                    if new_pos == 0 {
                        new_pos = input.0.len() - 1;
                    }

                    nums.remove(i);
                    nums.insert(new_pos, (x, n));
                    break;
                }
            }
        }
    }

    let cycled = nums
        .iter()
        .map(|(_, n)| n)
        .cycle()
        .copied()
        .skip_while(|n| *n != 0)
        .take(3001)
        .collect::<Vec<_>>();
    cycled[1000] + cycled[2000] + cycled[3000]
}

fn parse(reader: Vec<String>) -> Input {
    (
        reader.into_iter().map(|row| row.parse().unwrap()).collect(),
        811589153,
    )
}

pub fn twenty() -> Result<(), std::io::Error> {
    let file = File::open("input/20_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 20 part 1: {}", one_impl(&input));
    println!("Day 20 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_20::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
1
2
-3
3
-2
0
4
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(3, one_impl(&parsed));
        assert_eq!(1623178306, two_impl(&parsed));
    }
}
