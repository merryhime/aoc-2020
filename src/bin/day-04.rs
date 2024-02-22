use std::fs;

fn parse(input: &str) -> Vec<Vec<(&str, &str)>> {
    let re = regex::Regex::new("\n\n").unwrap();
    re.split(input)
        .map(|ps| {
            ps.split_ascii_whitespace()
                .map(|xs| xs.split_once(':').unwrap())
                .collect()
        })
        .collect()
}

fn is_valid(p: &[(&str, &str)]) -> bool {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    fields.iter().all(|f| p.iter().any(|(a, _)| a == f))
}

fn is_field_valid(field_name: &str, field_value: &str) -> bool {
    let num_value = field_value.parse::<u64>().ok();
    let trimmed_num_value = if field_value.len() > 2 {
        field_value
            .get(0..(field_value.len() - 2))
            .and_then(|x| x.parse::<u64>().ok())
    } else {
        None
    };

    match field_name {
        "byr" => num_value >= Some(1920) && num_value <= Some(2002),
        "iyr" => num_value >= Some(2010) && num_value <= Some(2020),
        "eyr" => num_value >= Some(2020) && num_value <= Some(2030),
        "hgt" => {
            if field_value.ends_with("cm") {
                trimmed_num_value >= Some(150) && trimmed_num_value <= Some(193)
            } else if field_value.ends_with("in") {
                trimmed_num_value >= Some(59) && trimmed_num_value <= Some(76)
            } else {
                false
            }
        }
        "hcl" => {
            field_value.get(0..1) == Some("#")
                && field_value
                    .chars()
                    .skip(1)
                    .all(|c| ('a'..='f').contains(&c) || ('0'..='9').contains(&c))
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&field_value),
        "pid" => field_value.len() == 9 && field_value.chars().all(|c| char::is_ascii_digit(&c)),
        _ => true,
    }
}

fn is_valid2(p: &[(&str, &str)]) -> bool {
    if !is_valid(p) {
        return false;
    }
    p.iter().all(|(n, v)| is_field_valid(n, v))
}

fn part1(s: &[Vec<(&str, &str)>]) -> usize {
    s.iter().filter(|p| is_valid(p)).count()
}

fn part2(s: &[Vec<(&str, &str)>]) -> usize {
    s.iter().filter(|p| is_valid2(p)).count()
}

fn main() {
    let example1 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    let input = fs::read_to_string("inputs/04").unwrap();

    let e = parse(example1);
    let i = parse(&input);

    println!("{:?}", part1(&e));
    println!("{:?}", part1(&i));
    println!("{:?}", part2(&e));
    println!("{:?}", part2(&i));
}
