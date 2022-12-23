use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Peekable;

#[derive(Clone, Debug, Default)]
pub struct Dir {
    files: Vec<(usize, String)>,
    dirs: HashMap<String, Dir>,
}

impl Dir {
    pub fn new() -> Dir {
        Dir::default()
    }
}

type Input = Dir;

fn sizes(input: &Dir) -> (usize, Vec<usize>) {
    let mut this = input.files.iter().map(|(size, _)| size).sum::<usize>();
    let mut inner: Vec<usize> = Vec::new();
    for d in input.dirs.values() {
        let (actual, dirs) = sizes(d);
        this += actual;
        inner.extend(&mut dirs.into_iter());
    }
    inner.push(this);
    (this, inner)
}

fn one_impl(input: &Input) -> usize {
    let (_, dirs) = sizes(input);
    dirs.iter().copied().filter(|s| *s <= 100000).sum()
}

fn two_impl(input: &Input) -> usize {
    let total = 70000000;
    let required = 30000000;
    let (actual, dirs) = sizes(input);
    dirs.iter()
        .copied()
        .filter(|s| total - actual + s > required)
        .min()
        .unwrap()
}

fn parse_<I>(fs: &mut Dir, iter: &mut Peekable<I>) -> Option<()>
where
    I: Iterator<Item = String>,
{
    while let Some(s) = iter.next() {
        if s.contains("cd") {
            let dir = s.split(' ').collect::<Vec<_>>()[2];
            if dir == "/" {
                return None;
            } else if dir == ".." {
                return Some(());
            } else {
                if !fs.dirs.contains_key(dir) {
                    fs.dirs.insert(dir.to_string(), Dir::new());
                }
                parse_(fs.dirs.get_mut(dir).unwrap(), iter)?;
            }
        } else if s.contains("ls") {
            while iter.peek().filter(|s| !s.contains('$')).is_some() {
                let next = iter.next().unwrap();
                let row = next.split(' ').collect::<Vec<_>>();
                if !row[0].starts_with("dir") {
                    fs.files.push((row[0].parse().unwrap(), row[1].to_string()));
                }
            }
        } else {
            unimplemented!("{}", s)
        }
    }
    None
}

fn parse(reader: Vec<String>) -> Input {
    let mut iter = reader.into_iter().peekable();
    let mut fs = Dir::new();
    while iter.peek().is_some() {
        parse_(&mut fs, &mut iter);
    }
    fs
}

pub fn seven() -> Result<(), std::io::Error> {
    let file = File::open("7_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 7 part 1: {}", one_impl(&input));
    println!("Day 7 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_7::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = [
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(95437, one_impl(&parsed));
        assert_eq!(24933642, two_impl(&parsed));
    }

    #[test]
    fn it_works_2() {
        let input = [
            "$ cd /",
            "$ cd a",
            "$ ls",
            "1 some.txt",
            "$ cd ..",
            "$ cd b",
            "$ cd a",
            "$ ls",
            "1 other.txt",
        ];
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(5, one_impl(&parsed));
    }

    #[test]
    fn it_works_3() {
        let input = [
            "$ cd /",
            "$ ls",
            "dir a",
            "dir b",
            "$ cd a",
            "$ ls",
            "1 first.txt",
            "1 second.txt",
            "$ cd ..",
            "$ cd b",
            "$ ls",
            "dir a",
            "$ cd a",
            "$ ls",
            "1 third.txt",
        ];
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(7, one_impl(&parsed));
    }
}
