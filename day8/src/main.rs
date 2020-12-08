use std::collections::HashSet;
use std::vec;
use utils::read_file;

#[derive(Debug, Clone)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

enum Status {
    LoopDetected,
    Completed,
    Running,
}

#[derive(Debug, Clone)]
struct Machine {
    instructions: vec::Vec<Op>,
    position: usize,
    called: HashSet<usize>,
    acc_store: i32,
}

fn op_from_str(line: &str) -> Op {
    let code = &line[0..3];
    match code {
        "acc" => Op::Acc(line[4..line.len()].parse::<i32>().unwrap()),
        "jmp" => Op::Jmp(line[4..line.len()].parse::<i32>().unwrap()),
        "nop" => Op::Nop(line[4..line.len()].parse::<i32>().unwrap()),
        _ => panic!("Unknown op code"),
    }
}

impl Machine {
    fn from_str(input: &str) -> Machine {
        let instructions = input.lines().map(op_from_str).collect();
        Machine::new(instructions)
    }

    fn new(instructions: vec::Vec<Op>) -> Machine {
        Machine {
            position: 0,
            instructions,
            called: HashSet::new(),
            acc_store: 0,
        }
    }

    fn step(&mut self) -> Status {
        let op_option = self.instructions.get(self.position);
        let pos_change_opt = match op_option {
            Some(op) => match op {
                Op::Acc(n) => {
                    self.acc_store += n;
                    Some(1)
                }
                Op::Jmp(n) => Some(*n),
                Op::Nop(_) => Some(1),
            },
            None => None,
        };
        self.called.insert(self.position);

        return match pos_change_opt {
            Some(pos_change) => {
                let new_pos = ((self.position as i32) + pos_change) as usize;

                if self.called.contains(&new_pos) {
                    return Status::LoopDetected;
                }
                self.position = new_pos;

                Status::Running
            }
            None => Status::Completed,
        };
    }

    fn run_to_completion(&mut self) -> Status {
        let mut status = self.step();
        while match status {
            Status::Running => true,
            _ => false,
        } {
            status = self.step();
        }
        status
    }

    fn reset(&mut self) {
        self.position = 0;
        self.acc_store = 0;
        self.called = HashSet::new()
    }
}

fn main() {
    let contents = read_file("/home/sam/src/github.com/sammorrowdrums/aoc2/input/day-8.txt");
    let mut machine = Machine::from_str(&contents);
    machine.run_to_completion();
    println!("Acc value {}", machine.acc_store);

    machine.reset();
    for (i, op) in machine.instructions.iter().enumerate() {
        match op {
            Op::Jmp(n) => {
                let mut new_machine = machine.clone();
                new_machine.instructions[i] = Op::Nop(*n);
                match new_machine.run_to_completion() {
                    Status::Completed => {
                        println!("Line {} needs to be changed", i + 1);
                        println!("Acc value {}", new_machine.acc_store);
                        break;
                    }
                    _ => continue,
                }
            }
            Op::Nop(n) => {
                let mut new_machine = machine.clone();
                new_machine.instructions[i] = Op::Jmp(*n);
                match new_machine.run_to_completion() {
                    Status::Completed => {
                        println!("Line {} needs to be changed", i + 1);
                        println!("Acc value {}", new_machine.acc_store);
                        break;
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
}
