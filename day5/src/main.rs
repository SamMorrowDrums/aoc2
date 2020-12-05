use utils::read_file;

#[derive(Debug, PartialEq)]
struct BoardingPass {
    row: u32,
    col: u32,
    id: u32,
}

impl BoardingPass {
    fn new(row: u32, col: u32) -> BoardingPass {
        BoardingPass {
            row,
            col,
            id: row * 8 + col,
        }
    }
}

fn main() {
    let contents = read_file("/home/sam/src/github.com/sammorrowdrums/aoc2/input/day-5.txt");
    let max_id = contents
        .lines()
        .map(|line| process_pass(line).id)
        .max()
        .unwrap();
    println!("Max ID is {}", max_id);

    let ids = contents
        .lines()
        .map(|line| process_pass(line).id)
        .collect::<Vec<_>>();

    for i in 8..max_id {
        if !ids.contains(&(i as u32))
            && ids.contains(&((i as u32) + 1))
            && ids.contains(&((i as u32) - 1))
        {
            println!("IDs surrounding {}", i);
            break;
        }
    }
}

fn derive_position(code: &str, min: u32, max: u32) -> u32 {
    let c = code.chars().nth(0);

    match c {
        Some(letter) => match letter {
            'F' => derive_position(&code[1..code.len()], min, max - ((1 + max - min) / 2)),
            'B' => derive_position(&code[1..code.len()], min + ((1 + max - min) / 2), max),
            'L' => derive_position(&code[1..code.len()], min, max - ((1 + max - min) / 2)),
            'R' => derive_position(&code[1..code.len()], min + ((1 + max - min) / 2), max),
            _ => panic!("Invalid Char"),
        },
        None => min,
    }
}

fn process_pass(pass_code: &str) -> BoardingPass {
    BoardingPass::new(
        derive_position(&pass_code[0..7], 0, 127),
        derive_position(&pass_code[7..10], 0, 7),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_pass() {
        let pass = "FBFBBFFRLR";
        assert_eq!(
            process_pass(pass),
            BoardingPass {
                row: 44,
                col: 5,
                id: 357
            }
        );
        let pass = "BFFFBBFRRR";
        assert_eq!(
            process_pass(pass),
            BoardingPass {
                row: 70,
                col: 7,
                id: 567
            }
        );
        let pass = "FFFBBBFRRR";
        assert_eq!(
            process_pass(pass),
            BoardingPass {
                row: 14,
                col: 7,
                id: 119
            }
        );
        let pass = "BBFFBBFRLL";
        assert_eq!(
            process_pass(pass),
            BoardingPass {
                row: 102,
                col: 4,
                id: 820
            }
        );
    }
}
