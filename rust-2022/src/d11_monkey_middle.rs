use std::io::{BufRead, BufReader};
use std::path::Path;
/// Day 11 Monkey in the middle

/// If the Option == None, it means we perform the operation on the old worry level value
#[derive(Eq, PartialEq, Clone, Debug)]
enum Op {
    Add(Option<usize>),
    Mul(Option<usize>),
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    item_op: Op,
    test_divisor: usize,
    test_true: usize,
    test_false: usize,
    inspection_count: usize,
}

fn read_input(path: &Path) -> Vec<Monkey> {
    let f = std::fs::File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut line_iter = reader.lines();
    let mut monkeys: Vec<Monkey> = vec![];

    while let Some(buf_line) = line_iter.next() {
        let line = buf_line.unwrap();
        if line.starts_with("Monkey") {
            let id = line.trim().split([' ', ':']).filter(|&s| s.starts_with(|c:char| c.is_ascii_digit())).last().unwrap();
            let id = id.parse::<usize>().unwrap();

            let items_line = line_iter.next().unwrap().unwrap();
            let items: Vec<_> = items_line.trim()
                .split([' ', ':', ','])
                .filter(|&s| s.starts_with(|c: char| c == '-' || c.is_ascii_digit()))
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            // parse operation line
            let op_line = line_iter.next().unwrap().unwrap();
            let tokens: Vec<_> = op_line.trim()
                .split(' ')
                .collect();
            let op_value: Option<usize> = match tokens[5] {
                "old" => None,
                s => Some(s.parse::<usize>().unwrap()),
            };
            let op: Op = match tokens[4] {
                "+" => Op::Add(op_value),
                "*" => Op::Mul(op_value),
                _ => panic!("unknown operation {}",tokens[4]),
            };

            // parse test divisible line
            let td_line = line_iter.next().unwrap().unwrap();
            let test_divisor = td_line.split(' ').last().unwrap().parse::<usize>().unwrap();

            // parse if true line
            let test_true_line = line_iter.next().unwrap().unwrap();
            let test_true = test_true_line.split(' ').last().unwrap().parse::<usize>().unwrap();

            // parse if false line
            let test_false_line = line_iter.next().unwrap().unwrap();
            let test_false = test_false_line.split(' ').last().unwrap().parse::<usize>().unwrap();

            monkeys.push(Monkey {
                id,
                items,
                item_op: op,
                test_divisor,
                test_true,
                test_false,
                inspection_count: 0
            })
        }
    }

    monkeys
}

/// Chasing all of the monkeys at once is impossible; you're going to have to focus on
/// the two most active monkeys if you want any hope of getting your stuff back. Count the
/// total number of times each monkey inspects items over 20 rounds:
/// Keep track of how many times a monkey inspected an item. The level of monkey business is the
/// product of the two monkeys with the highest inspection count
pub fn part1() {
    let mut monkeys = read_input(Path::new("../input-2022/d11-input.txt"));

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let true_id = monkeys[i].test_true;
            let false_id = monkeys[i].test_false;
            let mut true_items: Vec<usize> = vec![];
            let mut false_items: Vec<usize> = vec![];
            let item_len = monkeys[i].items.len();
            monkeys.get_mut(i).unwrap().inspection_count += item_len;

            for item in &monkeys[i].items {
                let mut worry = item.clone();
                match &monkeys[i].item_op {
                    Op::Add(Some(v)) => worry += v,
                    Op::Add(None) => worry += worry,
                    Op::Mul(Some(v)) => worry *= v,
                    Op::Mul(None) => worry *= worry,
                }
                let div3: usize = worry / 3;
                if div3 % &monkeys[i].test_divisor == 0 {
                    true_items.push(div3);
                } else {
                    false_items.push(div3);
                }
            }
            monkeys.get_mut(i).unwrap().items.clear();
            monkeys.get_mut(true_id).unwrap().items.append(&mut true_items);
            monkeys.get_mut(false_id).unwrap().items.append(&mut false_items);
        }
    }
    monkeys.sort_by(|m1, m2| m2.inspection_count.partial_cmp(&m1.inspection_count).unwrap() );
    let monkey_level: usize = monkeys.iter().take(2).map(|m| &m.inspection_count).product();
    dbg!(&monkeys);
    println!("monkey level {}", monkey_level);
}

/// part2 is like part1 but we don't divide worry levels by 3. And we do 10_000 rounds instead of 20
pub fn part2() {
    let mut monkeys = read_input(Path::new("../input-2022/d11-input.txt"));
    let divisor_product = monkeys.iter().map(|m| m.test_divisor).product::<usize>();

    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            let true_id = monkeys[i].test_true;
            let false_id = monkeys[i].test_false;
            let mut true_items: Vec<usize> = vec![];
            let mut false_items: Vec<usize> = vec![];
            let item_len = monkeys[i].items.len();
            monkeys.get_mut(i).unwrap().inspection_count += item_len;

            for item in &monkeys[i].items {
                let mut worry = item % divisor_product;
                match &monkeys[i].item_op {
                    Op::Add(Some(v)) => worry += v,
                    Op::Add(None) => worry += worry,
                    Op::Mul(Some(v)) => worry *= v,
                    Op::Mul(None) => worry *= worry,
                }

                if worry % &monkeys[i].test_divisor == 0 {
                    true_items.push(worry.clone());
                } else {
                    false_items.push(worry.clone());
                }
            }
            monkeys.get_mut(i).unwrap().items.clear();
            monkeys.get_mut(true_id).unwrap().items.append(&mut true_items);
            monkeys.get_mut(false_id).unwrap().items.append(&mut false_items);
        }
    }
    monkeys.sort_by(|m1, m2| m2.inspection_count.partial_cmp(&m1.inspection_count).unwrap() );
    let monkey_level: usize = monkeys.iter().take(2).map(|m| &m.inspection_count).product();
    dbg!(&monkeys);
    println!("monkey level {}", monkey_level);
}


#[cfg(test)]
mod tests {
    use crate::d11_monkey_middle::{Monkey, Op, part1, part2};

    #[test]
    fn test_part1() {
        part1();
    }

    #[test]
    fn test_part2() {
        part2();
    }

    #[test]
    fn test_monkey_line_parsing() {
        let line = "Monkey 0:";
        let tokens: Vec<_> = line.split([' ', ':']).collect();
        assert_eq!(tokens.len(), 3);
        assert!(tokens[1].parse::<usize>().is_ok());
    }

    #[test]
    fn test_items_line_parsing() {
        let line = "  Starting items: 76, -88, 96, 97, 58, 61, 67";
        let tokens: Vec<_> = line.trim()
            .split([' ', ':', ','])
            .filter(|&s| s.starts_with(|c: char| c == '-' || c.is_ascii_digit()))
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        assert_eq!(tokens.len(), 7);
    }

    #[test]
    fn test_operation_line_parsing_with_last_numeric() {
        let line = "  Operation: new = old * 19";
        let tokens: Vec<_> = line.trim()
            .split(' ')
            .collect();
        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[4], "*" | "+"));
        assert!(tokens[5].starts_with(|c: char| c == '-' || c.is_ascii_digit()));
        assert_eq!(tokens[5].parse::<usize>().unwrap(), usize::from(19_usize));
    }

    // no negative numbers in input file, this test is not needed
    // #[test]
    // fn test_operation_line_parsing_with_last_negative_numeric() {
    //     let line = "  Operation: new = old * -19";
    //     let tokens: Vec<_> = line.trim()
    //         .split(' ')
    //         .collect();
    //     assert_eq!(tokens.len(), 6);
    //     assert!(matches!(tokens[4], "*" | "+" | "-" | "/"));
    //     assert!(tokens[5].starts_with(|c: char| c == '-' || c.is_ascii_digit()));
    //     assert_eq!(tokens[5].parse::<u128>().unwrap(), -19);
    // }

    #[test]
    fn test_operation_line_parsing_with_last_eq_old() {
        let line = "  Operation: new = old * old";
        let tokens: Vec<_> = line.trim()
            .split(' ')
            .collect();
        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[4], "*" | "+"));
        assert_eq!(tokens[5], "old");
    }

    #[test]
    fn test_divisible_line_parsing() {
        let line = "  Test: divisible by 17";
        let tokens: Vec<_> = line.trim()
            .split(' ')
            .collect();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens.last().unwrap(), &"17");
    }

    #[test]
    fn test_if_true_line_parsing() {
        let line = "  If true: throw to monkey 2";
        let tokens: Vec<_> = line.trim()
            .split(' ')
            .collect();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[1], "true:");
        assert_eq!(tokens.last().unwrap(), &"2");
    }

    #[test]
    fn test_if_false_line_parsing() {
        let line = "  If false: throw to monkey 3";
        let tokens: Vec<_> = line.trim()
            .split(' ')
            .collect();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[1], "false:");
        assert_eq!(tokens.last().unwrap(), &"3");
    }

    #[test]
    fn test_input_parsing_two_groups() {
        let input = r"Monkey 0:
  Starting items: 76, 88, 96, 97, 58, 61, 67
  Operation: new = old * 19
  Test: divisible by 3
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 5:
  Starting items: 59, 62, 53, 62
  Operation: new = old * old
  Test: divisible by 7
    If true: throw to monkey 4
    If false: throw to monkey 7";

        let lines: Vec<&str> = input.lines().collect();
        let mut line_iter = lines.iter();
        let mut monkeys: Vec<Monkey> = vec![];
        while let Some(&line) = line_iter.next() {
            if line.starts_with("Monkey") {
                let id = line.trim().split([' ', ':']).filter(|&s| s.starts_with(|c:char| c.is_ascii_digit())).last().unwrap();
                let id = id.parse::<usize>().unwrap();

                let items_line = line_iter.next().unwrap();
                let items: Vec<_> = items_line.trim()
                    .split([' ', ':', ','])
                    .filter(|&s| s.starts_with(|c: char| c == '-' || c.is_ascii_digit()))
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();

                // parse operation line
                let op_line = line_iter.next().unwrap();
                let tokens: Vec<_> = op_line.trim()
                    .split(' ')
                    .collect();
                let op_value = match tokens[5] {
                    "old" => None,
                    s => Some(s.parse::<usize>().unwrap()),
                };
                let op: Op = match tokens[4] {
                    "+" => Op::Add(op_value),
                    "*" => Op::Mul(op_value),
                    _ => panic!("unknown operation {}",tokens[4]),
                };

                // parse test divisible line
                let td_line = line_iter.next().unwrap();
                let test_divisor = td_line.split(' ').last().unwrap().parse::<usize>().unwrap();

                // parse if true line
                let test_true_line = line_iter.next().unwrap();
                let test_true = test_true_line.split(' ').last().unwrap().parse::<usize>().unwrap();

                // parse if false line
                let test_false_line = line_iter.next().unwrap();
                let test_false = test_false_line.split(' ').last().unwrap().parse::<usize>().unwrap();

                monkeys.push(Monkey {
                    id,
                    items,
                    item_op: op,
                    test_divisor,
                    test_true,
                    test_false,
                    inspection_count: 0,
                })
            }
        }
        let monkey1 = Monkey { id: 0, items: vec![76, 88, 96, 97, 58, 61, 67], item_op: Op::Mul(Some(19)), test_divisor: 3, test_true: 2, test_false: 3, inspection_count: 0 };
        let monkey2 = Monkey { id: 5, items: vec![59, 62, 53, 62], item_op: Op::Mul(None), test_divisor: 7, test_true: 4, test_false: 7, inspection_count: 0 };
        assert_eq!(monkeys.len(), 2);
        assert_eq!(monkeys[0], monkey1);
        assert_eq!(monkeys[1], monkey2);
    }
}