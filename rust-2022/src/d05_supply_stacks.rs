use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

#[derive(Debug)]
struct MoveCommand {
    amount: usize,
    from: usize,
    to: usize,
}

impl MoveCommand {
    fn from(amount: usize, from: usize, to: usize) -> Self {
        Self {
            amount,
            from,
            to
        }
    }
}

/// initialize the stacks to their initial values
/// vec[0] is used as temp stack
fn init_stacks() -> Vec<Vec<char>> {
    vec![
        vec![],
        vec!['D', 'B', 'J', 'V'],
        vec!['P', 'V', 'B', 'W', 'R', 'D', 'F'],
        vec!['R', 'G', 'F', 'L', 'D', 'C', 'W', 'Q'],
        vec!['W', 'J', 'P', 'M', 'L', 'N', 'D', 'B'],
        vec!['H', 'N', 'B', 'P', 'C', 'S', 'Q'],
        vec!['R', 'D', 'B', 'S', 'N', 'G'],
        vec!['Z', 'B', 'P', 'M', 'Q', 'F', 'S', 'H'],
        vec!['W', 'L', 'F'],
        vec!['S', 'V', 'F', 'M', 'R']
    ]
}

/// performs a single move command
fn do_move(mc: &MoveCommand, stacks: &mut Vec<Vec<char>>) {
    for _ in 0..mc.amount {
        let c = stacks[mc.from].pop().expect("stack exists");
        stacks[mc.to].push(c);
    }
}

fn do_ordered_move(mc: &MoveCommand, stacks: &mut Vec<Vec<char>>) {
    for _ in 0..mc.amount {
        let c = stacks[mc.from].pop().expect("stack exists");
        stacks[0].push(c);
    }
    for _ in 0..stacks[0].len() {
        let c = stacks[0].pop().expect("stack 0 is not empty");
        stacks[mc.to].push(c);
    }
}

fn part1() {
    let path = Path::new("../input-2022/d05-input.txt");
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut stacks = init_stacks();

    for line in reader.lines() {
        let command_str = line.unwrap();
        let caps = MOVE_RE.captures(&command_str).unwrap();
        let move_command = MoveCommand::from(
            caps[1].parse::<usize>().unwrap(),
            caps[2].parse::<usize>().unwrap(),
            caps[3].parse::<usize>().unwrap(),
        );
        do_move(&move_command, &mut stacks);
    }
    println!("crates on top of each stack are:");
    for i in 1..stacks.len() {
        print!("{} ", stacks[i].last().unwrap())
    }
}

fn part2() {
    let path = Path::new("../input-2022/d05-input.txt");
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut stacks = init_stacks();

    for line in reader.lines() {
        let command_str = line.unwrap();
        let caps = MOVE_RE.captures(&command_str).unwrap();
        let move_command = MoveCommand::from(
            caps[1].parse::<usize>().unwrap(),
            caps[2].parse::<usize>().unwrap(),
            caps[3].parse::<usize>().unwrap(),
        );
        do_ordered_move(&move_command, &mut stacks);
    }
    println!("part 2 crates on top of each stack are:");
    for i in 1..stacks.len() {
        print!("{}", stacks[i].last().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use regex::Regex;
    use crate::d05_supply_stacks::{do_move, init_stacks, MoveCommand, part1, part2};

    #[test]
    fn do_part1() {
        part1();
    }

    #[test]
    fn do_part2() {
        part2();
    }

    #[test]
    fn test_move_re_captures() {
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let input = "move 2 from 3 to 4";
        let caps = re.captures(input).unwrap();
        assert!(re.is_match(input));
        assert_eq!(caps.len(), 4);

        assert_eq!(&caps[1], "2");
        assert_eq!(&caps[2], "3");
        assert_eq!(&caps[3], "4");
    }

    #[test]
    fn test_one_move() {
        let mut stack = init_stacks();
        let mc = MoveCommand::from(1, 4, 1);
        do_move(&mc, &mut stack);
        assert_eq!(stack[1], vec!['D', 'B', 'J', 'V', 'B']);
        assert_eq!(stack[4], vec!['W', 'J', 'P', 'M', 'L', 'N', 'D']);
    }
}