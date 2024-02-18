use std::fs;

fn parse_nums(str: &str) -> Vec<u64> {
    str.split('\n')
        .filter(|x| !x.is_empty())
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn part1(str: &str) -> u64 {
    let nums = parse_nums(str);
    for (i, x) in nums.iter().enumerate() {
        for y in nums.iter().skip(i + 1) {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    unreachable!("No solution");
}

fn part2(str: &str) -> u64 {
    let nums = parse_nums(str);
    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate().skip(i + 1) {
            for z in nums.iter().skip(j + 1) {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    unreachable!("No solution");
}

fn main() {
    let example1 = "1721
979
366
299
675
1456";
    println!("{}", part1(example1));
    println!("{}", part1(&fs::read_to_string("inputs/01").unwrap()));
    println!("{}", part2(example1));
    println!("{}", part2(&fs::read_to_string("inputs/01").unwrap()));
}
