use std::collections::HashSet;
use std::vec;
use utils::read_file;

fn main() {
    let contents = read_file("/home/sam/src/github.com/sammorrowdrums/aoc2/input/day-6.txt");
    println!(
        "Count of unique group answers {}",
        process_groups(&contents)
    );

    println!(
        "Count of unique group answers {}",
        process_groups_2(&contents)
    );
}

fn process_groups(data: &str) -> usize {
    data.split("\n\n")
        .map(|x| {
            let set = x
                .lines()
                .flat_map(|y| y.trim().chars())
                .collect::<HashSet<_>>();
            set.len()
        })
        .sum()
}

fn process_groups_2(data: &str) -> usize {
    data.split("\n\n")
        .map(|x| {
            let mut sets = x.lines().map(|y| y.trim().chars().collect::<HashSet<_>>());

            let intersection = sets
                .next()
                .map(|set| {
                    sets.fold(set, |set1, set2| {
                        set1.intersection(&set2).cloned().collect()
                    })
                })
                .unwrap();
            intersection.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "abc

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

    #[test]
    fn test_process_pass() {
        let n = process_groups(TEST_DATA);
        assert_eq!(n, 11);
    }
}
