use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::path::Path;

/// parse a pair of integers, separated by a single hyphen into a RangeInclusive<i32>.
/// ex.  3-8  becomes 3..=8
fn parse_as_range(s: &str) -> RangeInclusive<i32> {
    let mut splits = s.split("-");
    let start = splits.next().expect("input is well-formed").parse::<i32>().unwrap();
    let end = splits.next().expect("input is well-formed").parse::<i32>().unwrap();
    RangeInclusive::new(start, end)
}

/// returns `true` if r2 is completely contained within r1
fn contains(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    r1.contains(r2.start()) && r1.contains(r2.end())
}

/// returns `true` if r1 contains any portion of r2
fn overlaps(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    r1.contains(r2.start()) || r1.contains(r2.end())
}

/// find fully contained pairs
fn part1() {
    let path = Path::new("../input/d04-input.txt");
    let f = File::open(path).expect("input file can be found");
    let reader = BufReader::new(f);

    let fully_contained_pairs = reader.lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let mut splits = line.split(",");
            let left = splits.next().unwrap();
            let right = splits.next().unwrap();
            let left_range = parse_as_range(left);
            let right_range = parse_as_range(right);
            contains(&left_range, &right_range) || contains(&right_range, &left_range)
        })
        .filter(|res| *res)
        .count();
    println!("total fully contained pairs = {}", fully_contained_pairs);
}

/// find partially contained pairs
fn part2() {
    let path = Path::new("../input/d04-input.txt");
    let f = File::open(path).expect("input file can be found");
    let reader = BufReader::new(f);

    let fully_contained_pairs = reader.lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let mut splits = line.split(",");
            let left = splits.next().unwrap();
            let right = splits.next().unwrap();
            let left_range = parse_as_range(left);
            let right_range = parse_as_range(right);
            overlaps(&left_range, &right_range) || overlaps(&right_range, &left_range)
        })
        .filter(|res| *res)
        .count();
    println!("total fully contained pairs = {}", fully_contained_pairs);
}


#[cfg(test)]
mod tests {
    use crate::d04_camp_cleanup::{contains, overlaps, parse_as_range, part1, part2};

    #[test]
    fn do_part1() {
        part1()
    }

    #[test]
    fn do_part2() {
        part2()
    }


    #[test]
    fn test_parse_as_range() {
        let s = "2-8";
        let r = parse_as_range(s);
        assert_eq!(r.start(), &2);
        assert_eq!(r.end(), &8);
    }

    #[test]
    fn test_contains() {
        let r1 = 2..=10;
        let r2 = 3..=9;
        assert!(contains(&r1, &r2));
    }

    #[test]
    fn test_contains_end_bound() {
        let r1 = 2..=10;
        let r2 = 3..=10;
        assert!(contains(&r1, &r2));
    }

    #[test]
    fn test_contains_same_bound() {
        let r1 = 2..=10;
        let r2 = 2..=10;
        assert!(contains(&r1, &r2));
    }

    #[test]
    fn test_not_contains() {
        let r1 = 2..=10;
        let r2 = 1..=10;
        assert!(contains(&r1, &r2));
    }

    #[test]
    fn test_not_contains_end_bound() {
        let r1 = 2..=10;
        let r2 = 3..=12;
        assert!(contains(&r1, &r2));
    }

    #[test]
    fn test_overlaps() {
        let r1 = 2..=5;
        let r2 = 3..=6;
        assert!(overlaps(&r1, &r2));
    }

    #[test]
    fn test_overlaps2() {
        let r1 = 2..=5;
        let r2 = 3..=6;
        assert!(overlaps(&r2, &r1));
    }

    #[test]
    fn test_no_overlaps() {
        let r1 = 2..=5;
        let r2 = 7..=10;
        assert!(!overlaps(&r2, &r1));
    }
}