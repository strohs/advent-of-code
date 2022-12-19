use std::collections::HashMap;
use std::fs::File;
use std::path::{PathBuf};
use std::io::{BufRead, BufReader};
use regex::Regex;

// first half answer: 11926135976176


fn parse_mask(mask: &str) -> String {
    let ms = mask.split(" = ").last().unwrap();
    String::from("0000000000000000000000000000") + ms
}

fn parse_mem(line: &str) -> (usize, u64) {
    let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let caps = re.captures(line).unwrap();
    let index = caps.get(1).unwrap().as_str();
    let amount = caps.get(2).unwrap().as_str();
    (usize::from_str_radix(index, 10).unwrap(), u64::from_str_radix(amount, 10).unwrap())
}

fn merge(mask: &str, value: u64) -> usize {
    let value = format!("{:064b}", value);
    let merged: String = value.chars().zip(mask.chars()).map(|(v, m)| {
        if m == 'X' {
            v
        } else {
            m
        }
    })
        .collect();
    usize::from_str_radix(&merged, 2).unwrap()
}


fn main() {
    let path = PathBuf::from("./input/14-input.txt");
    let file = File::open(path).expect("input file exists");

    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut cur_mask = "0".repeat(64);
    for line in BufReader::new(file).lines() {
        match line {
            Ok(l) if l.starts_with("mask") => {
                cur_mask = parse_mask(&l);
            },
            Ok(l) if l.starts_with("mem") => {
                let (index, amt) = parse_mem(&l);
                let merged = merge(&cur_mask, amt);
                map.insert(index, merged);
            },
            Err(e) => eprintln!("{}", e),
            l => eprintln!("unmatched line {:?}", l)
        }
    }

    let sum: usize = map.values().sum();
    println!("total sum {}", sum);
}

#[cfg(test)]
mod tests {
    use crate::{merge, parse_mask, parse_mem};

    #[test]
    fn basic_mask() {
        let mask_raw = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let mask = parse_mask(mask_raw);

        let (index, amt) = parse_mem("mem[8] = 11");
        assert_eq!(index, 8);
        assert_eq!(amt, 11);

        assert_eq!(merge(&mask, 11), 73);
    }
}