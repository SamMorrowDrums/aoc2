use utils::read_file;
use combinations::Combinations;
use std::vec;

fn combinations_add_to(values: &Vec<i32>, n_items: usize, target: i32) -> Vec<vec::Vec<i32>> {
    Combinations::new(values.to_vec(), n_items)
        .filter(|x| x.iter().sum::<i32>() == target)
        .collect()
}

fn main() {
    let contents = read_file("/home/sam/src/github.com/sammorrowdrums/aoc2/input/day-1.txt");
    let numbers: vec::Vec<i32> = contents
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let output = combinations_add_to(&numbers, 2, 2020);
    for value in output {
        println!("Part 1: {}", value.iter().product::<i32>());
    }

    let output = combinations_add_to(&numbers, 3, 2020);
    for value in output {
        println!("Part 2: {}", value.iter().product::<i32>());
    }
}
