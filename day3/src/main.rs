use std::thread::current;
use std::vec;
use utils::read_file;

const PATTERNS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

#[derive(Debug, PartialEq)]
struct Grid {
    max_x: usize,
    max_y: usize,
    data: vec::Vec<vec::Vec<bool>>,
}

impl Grid {
    fn new(data: vec::Vec<vec::Vec<bool>>) -> Grid {
        let max_x = data[0].len();
        let max_y = data.len();
        Grid { data, max_x, max_y }
    }

    fn from_string(text: &str) -> Grid {
        Grid::new(
            text.lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect(),
        )
    }

    fn get_pos(&self, x: usize, y: usize) -> bool {
        self.data[y % self.max_y][x % self.max_x]
    }
}

fn follow_pattern(grid: &Grid, origin: (usize, usize), step: (usize, usize)) -> usize {
    let mut current_x_pos = origin.0;
    let mut current_y_pos = origin.1;
    let mut trees = 0;

    while current_y_pos < grid.max_y {
        if grid.get_pos(current_x_pos, current_y_pos) {
            trees += 1;
        }
        current_x_pos += step.0;
        current_y_pos += step.1;
    }

    trees
}

fn main() {
    let contents = read_file("/home/sam/src/github.com/sammorrowdrums/aoc2/input/day-3.txt");
    let grid = Grid::from_string(&contents);
    let n_trees = follow_pattern(&grid, (0, 0), (3, 1));
    println!("Number of trees: {:?}", n_trees);
    let answer = PATTERNS
        .iter()
        .map(|p| follow_pattern(&grid, (0, 0), *p))
        .product::<usize>();
    println!("Product of trees: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..##.......
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

    #[test]
    fn test_parse() {
        let grid = Grid::from_string(TEST_INPUT);
        assert_eq!(grid.get_pos(0, 0), false);
        assert_eq!(grid.get_pos(2, 0), true);
        assert_eq!(grid.get_pos(10, 10), true);
        assert_eq!(grid.get_pos(11, 0), false);
        assert_eq!(grid.get_pos(0, 11), false);
        assert_eq!(grid.get_pos(20, 20), false);
    }

    #[test]
    fn test_pattern_follow() {
        let grid = Grid::from_string(TEST_INPUT);
        let n_trees = follow_pattern(&grid, (0, 0), (3, 1));
        assert_eq!(n_trees, 7);
    }

    #[test]
    fn test_multi_pattern_follow() {
        let grid = Grid::from_string(TEST_INPUT);

        let answer = PATTERNS
            .iter()
            .map(|p| follow_pattern(&grid, (0, 0), *p))
            .product::<usize>();
        assert_eq!(answer, 336);
    }
}
