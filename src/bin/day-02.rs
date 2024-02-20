use std::fs;

fn is_valid(str: &str) -> Option<bool> {
    if let [min, max, ch, _, s] = str.split(&[' ', ':', '-'][..]).collect::<Vec<_>>()[..] {
        let min = min.parse::<usize>().ok()?;
        let max = max.parse::<usize>().ok()?;
        let count = s.split("").filter(|&c| c == ch).count();
        Some(min <= count && count <= max)
    } else {
        println!("{str}");
        None
    }
}

fn is_valid2(str: &str) -> Option<bool> {
    if let [a, b, ch, _, s] = str.split(&[' ', ':', '-'][..]).collect::<Vec<_>>()[..] {
        let a = a.parse::<usize>().ok()?;
        let b = b.parse::<usize>().ok()?;
        let chars = s.split("").collect::<Vec<_>>();
        Some([a, b].iter().filter(|&&i| chars[i] == ch).count() == 1)
    } else {
        println!("{str}");
        None
    }
}

fn part1(str: &str) -> usize {
    str.split_terminator('\n')
        .filter(|x| is_valid(x).unwrap())
        .count()
}

fn part2(str: &str) -> usize {
    str.split_terminator('\n')
        .filter(|x| is_valid2(x).unwrap())
        .count()
}

fn main() {
    let example1 = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    println!("{}", part1(example1));
    println!("{}", part1(&fs::read_to_string("inputs/02").unwrap()));
    println!("{}", part2(example1));
    println!("{}", part2(&fs::read_to_string("inputs/02").unwrap()));
}
