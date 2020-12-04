#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::vec;
use utils::read_file;

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn parse(
    data: &str,
    validator: &dyn Fn(&HashMap<String, String>) -> bool,
) -> Vec<HashMap<String, String>> {
    let any_whitespace = Regex::new(r"\s+").unwrap();
    data.split("\n\n")
        .map(|p| {
            any_whitespace
                .split(p)
                .filter(|x| x.contains(':'))
                .map(|entry| {
                    let index = entry.find(':').unwrap();
                    (
                        entry[0..index].to_string(),
                        entry[index + 1..entry.len()].to_string(),
                    )
                })
                .collect::<HashMap<_, _>>()
        })
        .filter(validator)
        .collect()
}

fn main() {
    let contents = read_file("/home/sam/src/github.com/sammorrowdrums/aoc2/input/day-4.txt");
    let x = parse(&contents, &validator);
    println!("Total valid passports {:?}", x.len());
    let x = parse(&contents, &validator2);
    println!("Total fully valid passports {:?}", x.len());
}

fn validator(item: &HashMap<String, String>) -> bool {
    for key in &REQUIRED_KEYS {
        if !item.contains_key(*key) {
            return false;
        }
    }
    true
}

fn parse_min_max(n: &String, min_n: u16, max_n: u16) -> bool {
    match n.parse::<u16>() {
        Ok(x) => x >= min_n && x <= max_n,
        _ => false,
    }
}

fn parse_height_min_max(n: &String) -> bool {
    if n.ends_with("cm") {
        parse_min_max(&n[0..n.find('c').unwrap()].to_string(), 150, 193)
    } else if n.ends_with("in") {
        parse_min_max(&n[0..n.find('i').unwrap()].to_string(), 59, 76)
    } else {
        false
    }
}

fn validate_hair_color(n: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#[\d|a-f]{6}").unwrap();
    }
    RE.is_match(n)
}

fn validator2(item: &HashMap<String, String>) -> bool {
    for key in &REQUIRED_KEYS {
        if !item.contains_key(*key) {
            return false;
        }
    }

    let byr = item.get("byr").unwrap();
    let iyr = item.get("iyr").unwrap();
    let eyr = item.get("eyr").unwrap();
    let hgt = item.get("hgt").unwrap();
    let hcl = item.get("hcl").unwrap();
    let ecl = item.get("ecl").unwrap();
    let pid = item.get("pid").unwrap();

    if !parse_min_max(byr, 1920, 2002)
        || !parse_min_max(iyr, 2010, 2020)
        || !parse_min_max(eyr, 2020, 2030)
        || !parse_height_min_max(hgt)
        || !validate_hair_color(hcl)
        || !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&**ecl)
        || !(pid.len() == 9 && pid.parse::<u32>().is_ok())
    {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const INVALID_INPUT: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID_INPUT: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn test_parse() {
        let x = parse(TEST_INPUT, &validator);
        assert_eq!(2, x.len());
    }

    #[test]
    fn test_valid() {
        let x = parse(VALID_INPUT, &validator2);
        assert_eq!(4, x.len());
    }

    #[test]
    fn test_invalid() {
        let x = parse(INVALID_INPUT, &validator2);
        assert_eq!(0, x.len());
    }
}
