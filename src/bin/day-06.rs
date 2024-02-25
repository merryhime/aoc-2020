use counter::Counter;
use itertools::Itertools;
use std::fs;

fn count_qns_in_group(s: &str) -> usize {
    s.chars()
        .filter(char::is_ascii_alphabetic)
        .unique()
        .count()
}

fn part1(s: &str) -> usize {
    let re = regex::Regex::new(r"\r?\n\r?\n").unwrap();
    re.split(s).map(count_qns_in_group).sum()
}

fn count_occurances_in_group(s: &str) -> usize {
    let c = s
        .chars()
        .filter(char::is_ascii_alphabetic)
        .collect::<Counter<_>>();
    let num = s.lines().count();
    c.iter().filter(|(_, &v)| v == num).count()
}

fn part2(s: &str) -> usize {
    let re = regex::Regex::new(r"\r?\n\r?\n").unwrap();
    re.split(s).map(count_occurances_in_group).sum()
}

fn main() {
    let example1 = "abc

a
b
c

ab
ac

a
a
a
a

b";

    println!("{}", part1(example1));
    println!("{}", part1(&fs::read_to_string("inputs/06").unwrap()));
    println!("{}", part2(example1));
    println!("{}", part2(&fs::read_to_string("inputs/06").unwrap()));
}
