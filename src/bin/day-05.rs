use std::fs;

fn parse(s: &str) -> u64 {
    let s = s
        .replace('F', "0")
        .replace('B', "1")
        .replace('L', "0")
        .replace('R', "1");
    u64::from_str_radix(&s, 2).expect(&s)
}

fn part1(ss: &str) -> u64 {
    ss.lines().map(parse).max().unwrap()
}

fn part2(ss: &str) -> u64 {
    let mut seats: Vec<u64> = ss.lines().map(parse).collect();
    seats.sort_unstable();
    println!("{seats:?}");
    let first = *seats.first().unwrap();
    let last = *seats.last().unwrap();
    std::iter::zip(first..last, seats.iter())
        .find(|(a, b)| a != *b)
        .unwrap()
        .0
}

fn main() {
    let example1 = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    println!("{}", part1(example1));
    println!("{}", part1(&fs::read_to_string("inputs/05").unwrap()));
    println!("{}", part2(&fs::read_to_string("inputs/05").unwrap()));
}
