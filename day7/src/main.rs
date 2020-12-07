#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec;
use utils::read_file;

#[derive(Debug, PartialEq)]
struct BagRule {
    color: String,
    n: u32,
}

impl BagRule {
    fn new(color: &str, n: u32) -> BagRule {
        BagRule {
            color: color.to_string(),
            n,
        }
    }

    fn parse(text: &str) -> Option<BagRule> {
        if text == "no other bags" {
            None
        } else {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"(\d+)\s([\w\s]+)\sbags?").unwrap();
            }
            let caps = RE.captures(text).unwrap();
            let n = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let col = caps.get(2).unwrap().as_str();
            Some(BagRule::new(col, n))
        }
    }
}

#[derive(Debug, PartialEq)]
struct BagRuleContainer {
    color: String,
    rules: vec::Vec<BagRule>,
}

impl BagRuleContainer {
    fn can_contain(&self, query: &str) -> bool {
        self.rules.iter().filter(|x| x.color == query).count() > 0
    }

    fn new(color: &str, rules: vec::Vec<BagRule>) -> BagRuleContainer {
        BagRuleContainer {
            color: color.to_string(),
            rules,
        }
    }

    fn from_rule_data(rule_data: &str) -> BagRuleContainer {
        let mut segments = rule_data.split(" bags contain ");
        let col = segments.next().unwrap();
        let text = segments.next().unwrap();
        let rules_vec = text[0..text.len() - 1]
            .split(", ")
            .filter_map(BagRule::parse)
            .collect();
        BagRuleContainer::new(col, rules_vec)
    }
}

fn count_containing_bags(rules: &vec::Vec<BagRuleContainer>, start: &str) -> usize {
    let mut lookup = vec![start.to_string()];

    let mut bags = HashSet::new();

    while !lookup.is_empty() {
        let case = lookup.pop().unwrap();
        for rule in rules {
            if rule.can_contain(&case) {
                bags.insert(rule.color.clone());
                lookup.push(rule.color.clone());
            }
        }
    }
    bags.len()
}

fn count_inner_bags(rules: &vec::Vec<BagRuleContainer>, start: &str) -> u32 {
    let lookup = rules
        .iter()
        .map(|r| (r.color.clone(), &r.rules))
        .collect::<HashMap<_, _>>();
    let mut count: u32 = 1;
    let inner = *lookup.get(start).unwrap();
    count += inner
        .iter()
        .map(|r| r.n * count_inner_bags(rules, &r.color))
        .sum::<u32>();
    return count;
}

fn main() {
    let contents = read_file("/home/sam/src/github.com/sammorrowdrums/aoc2/input/day-7.txt");

    let rules = parse_rules(&contents);
    let count = count_containing_bags(&rules, "shiny gold");

    println!("Count of bags that can contain shiny gold bags {}", count);

    let count = count_inner_bags(&rules, "shiny gold");
    println!(
        "Count of bags that can could be contained by a shiny gold bag {}",
        count - 1
    );
}

fn parse_rules(data: &str) -> vec::Vec<BagRuleContainer> {
    data.lines().map(BagRuleContainer::from_rule_data).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const TEST_DATA_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_parse_rules() {
        let rules = parse_rules(TEST_DATA);
        assert_eq!(rules[0].rules[1].color, "muted yellow");
        assert_eq!(rules[0].rules[1].n, 2);
        assert_eq!(rules[7].rules, vec![]);
    }

    #[test]
    fn test_how_many_bags() {
        let rules = parse_rules(TEST_DATA);

        let count = count_containing_bags(&rules, "shiny gold");

        assert_eq!(count, 4);
    }

    #[test]
    fn test_how_many_inner_bags() {
        let rules = parse_rules(TEST_DATA_2);
        let count = count_inner_bags(&rules, "shiny gold");

        assert_eq!(count - 1, 126);
    }
}
