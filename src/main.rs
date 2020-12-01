use combinations::Combinations;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec;

fn main() {
    // Create a path to the desired file
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

fn combinations_add_to(values: &Vec<i32>, n_items: usize, target: i32) -> Vec<vec::Vec<i32>> {
    Combinations::new(values.to_vec(), n_items)
        .filter(|x| x.iter().sum::<i32>() == target)
        .collect()
}

fn read_file(location: &str) -> String {
    let path = Path::new(location);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => return s,
    }
}
