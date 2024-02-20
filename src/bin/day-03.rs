use std::fs;
use std::ops::AddAssign;

struct Map<'a> {
    data: Vec<&'a [u8]>,
    height: usize,
}

#[derive(Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Map<'_> {
    fn init<'a>(data: &'a str) -> Map<'a> {
        let data: Vec<&[u8]> = data.lines().map(str::as_bytes).collect();
        let height = data.len();
        Map { data, height }
    }

    fn width(&self, y: usize) -> usize {
        self.data[y].len()
    }

    fn read(&self, c: Coord) -> Option<u8> {
        if c.y >= self.height {
            return None;
        }
        Some(self.data[c.y][c.x % self.width(c.y)])
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn count(map: &Map, step: Coord) -> usize {
    let mut tree_count = 0;
    let mut pos = Coord { x: 0, y: 0 };
    loop {
        match map.read(pos) {
            None => break,
            Some(b'#') => tree_count += 1,
            Some(_) => {}
        }
        pos += step;
    }
    tree_count
}

fn part1(map: &Map) -> usize {
    count(map, Coord { x: 3, y: 1 })
}

fn part2(map: &Map) -> usize {
    let steps = [
        Coord { x: 1, y: 1 },
        Coord { x: 3, y: 1 },
        Coord { x: 5, y: 1 },
        Coord { x: 7, y: 1 },
        Coord { x: 1, y: 2 },
    ];
    steps.iter().map(|&s| count(map, s)).product()
}

fn main() {
    let example1 = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    let emap = Map::init(example1);
    let input = fs::read_to_string("inputs/03").unwrap();
    let imap = Map::init(&input);

    println!("{}", part1(&emap));
    println!("{}", part1(&imap));
    println!("{}", part2(&emap));
    println!("{}", part2(&imap));
}
