use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = Vec<Vec<char>>;

fn snafu_to_num(snafu: &[char]) -> i64 {
    let mut num = 0;
    for c in snafu.iter().copied() {
        num *= 5;
        num += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unimplemented!(),
        };
    }
    num
}

fn num_to_snafu(mut num: i64) -> String {
    let mut res = Vec::new();
    while num > 0 {
        let mut carry = 0;
        let d = num % 5;
        match d {
            0 => {
                res.push('0');
            }
            1 => {
                res.push('1');
            }
            2 => {
                res.push('2');
            }
            3 => {
                res.push('=');
                carry = 1;
            }
            4 => {
                res.push('-');
                carry = 1;
            }
            _ => unreachable!(),
        }
        num /= 5;
        num += carry;
    }
    res.into_iter().rev().collect()
}

fn one_impl(input: &Input) -> (i64, String) {
    let mut sum = 0;

    for row in input {
        sum += snafu_to_num(row);
    }

    (sum, num_to_snafu(sum))
}

fn parse(reader: Vec<String>) -> Input {
    reader
        .into_iter()
        .map(|row| row.chars().collect())
        .collect()
}

pub fn twentyfive() -> Result<(), std::io::Error> {
    let file = File::open("25_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 25: {:?}", one_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_25::{num_to_snafu, one_impl, parse, snafu_to_num};

    #[test]
    fn t_snafu_to_num() {
        let f = |s: &str| snafu_to_num(&s.chars().collect::<Vec<_>>());
        assert_eq!(1, f("1"));
        assert_eq!(2, f("2"));
        assert_eq!(3, f("1="));
        assert_eq!(4, f("1-"));
        assert_eq!(5, f("10"));
        assert_eq!(6, f("11"));
        assert_eq!(7, f("12"));
        assert_eq!(8, f("2="));
        assert_eq!(9, f("2-"));
        assert_eq!(10, f("20"));
        assert_eq!(15, f("1=0"));
        assert_eq!(20, f("1-0"));
        assert_eq!(2022, f("1=11-2"));
        assert_eq!(12345, f("1-0---0"));
        assert_eq!(314159265, f("1121-1110-1=0"));
    }

    #[test]
    fn t_num_to_snafu() {
        assert_eq!(num_to_snafu(1), "1");
        assert_eq!(num_to_snafu(2), "2");
        assert_eq!(num_to_snafu(3), "1=");
        assert_eq!(num_to_snafu(4), "1-");
        assert_eq!(num_to_snafu(5), "10");
        assert_eq!(num_to_snafu(6), "11");
        assert_eq!(num_to_snafu(7), "12");
        assert_eq!(num_to_snafu(8), "2=");
        assert_eq!(num_to_snafu(9), "2-");
        assert_eq!(num_to_snafu(10), "20");
        assert_eq!(num_to_snafu(15), "1=0");
        assert_eq!(num_to_snafu(20), "1-0");
        assert_eq!(num_to_snafu(2022), "1=11-2");
        assert_eq!(num_to_snafu(12345), "1-0---0");
        assert_eq!(num_to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn it_works() {
        let input = "
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!((4890, "2=-1=0".to_string()), one_impl(&parsed));
    }
}
