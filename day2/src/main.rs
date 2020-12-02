use regex::Regex;
use std::vec;
use utils::read_file;

#[derive(Debug, PartialEq)]
struct Result {
    min_n: usize,
    max_n: usize,
    c: char,
    data: String,
}

impl Result {
    fn new(min_n: usize, max_n: usize, c: char, data: String) -> Result {
        Result {
            min_n,
            max_n,
            c,
            data,
        }
    }
}

fn main() {
    let contents = read_file("/home/sam/src/github.com/sammorrowdrums/aoc2/input/day-2.txt");
    let results = parse_day2_input(&contents);
    let valid = check_passwords(results);
    println!("Number of valid passwords: {}", valid.len());

    let results = parse_day2_input(&contents);
    let valid = check_passwords_2(results);
    println!("Part 2 Number of valid passwords: {}", valid.len());
}

fn parse_day2_input(input: &str) -> vec::Vec<Result> {
    let re = Regex::new(r"(?m)^(\d+)-(\d+)\s(\w):\s(\w+)$").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            Result::new(
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
                cap[3].chars().next().unwrap(),
                cap[4].to_string(),
            )
        })
        .collect()
}

fn check_passwords_2(input: vec::Vec<Result>) -> vec::Vec<Result> {
    input.into_iter().filter(is_valid_password_2).collect()
}

fn check_passwords(input: vec::Vec<Result>) -> vec::Vec<Result> {
    input.into_iter().filter(is_valid_password).collect()
}

fn is_valid_password(r: &Result) -> bool {
    let count = r.data.matches(r.c).count();
    if count >= r.min_n && count <= r.max_n {
        return true;
    }
    return false;
}

fn is_valid_password_2(r: &Result) -> bool {
    let p1 = r.data.chars().nth(r.min_n - 1);
    let p2 = r.data.chars().nth(r.max_n - 1);

    match (p1, p2) {
        (Some(x), Some(y)) => {
            if x == r.c && y != r.c {
                true
            } else if y == r.c && x != r.c {
                true
            } else {
                false
            }
        }
        (Some(x), None) => x == r.c,
        (None, Some(x)) => x == r.c,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_parse() {
        let parsed = parse_day2_input(TEST_INPUT);

        assert_eq!(Result::new(1, 3, 'a', "abcde".to_string()), parsed[0]);
        assert_eq!(Result::new(1, 3, 'b', "cdefg".to_string()), parsed[1]);
        assert_eq!(Result::new(2, 9, 'c', "ccccccccc".to_string()), parsed[2]);
    }

    #[test]
    fn test_rules() {
        let parsed = parse_day2_input(TEST_INPUT);
        let valid = check_passwords(parsed);
        assert_eq!(2, valid.len());
        assert_eq!(valid[0].data, "abcde");
        assert_eq!(valid[1].data, "ccccccccc");
    }

    #[test]
    fn test_rules_2() {
        let parsed = parse_day2_input(TEST_INPUT);
        let valid = check_passwords_2(parsed);
        assert_eq!(1, valid.len());
        assert_eq!(valid[0].data, "abcde");
    }
}
