use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

fn count_calories_p1(file: &Path) -> Vec<i32> {
    let f = File::open(file).expect("input file can be found");
    let reader = BufReader::new(f);

    let mut sum = 0_i32;
    let mut sums = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) if l.is_empty() => {
                sums.push(sum);
                sum = 0;
            },
            Ok(l) =>
                sum += l.parse::<i32>().expect("valid i32"),
            Err(e) =>
                panic!("error reading input file {}", e)
        }
    }

    // find max element, retaining the index of that element
    let max = sums
        .iter()
        .enumerate()
        .max_by(|&(_x_idx, x_val), &(_y_idx, y_val)| x_val.cmp(y_val))
        .expect("iterator should have at least one element");
    println!("elf with max calories is {} with calories = {}", max.0 + 1, max.1);


    sums
}

fn find_top_three(calories: &mut [i32]) -> i32 {
    calories.sort_by(|x, y| y.cmp(x));
    let top3: Vec<&i32> = calories.iter().take(3).collect();
    dbg!(&top3);
    let top3_sum: i32 = top3.into_iter().sum();
    println!("sum of max 3 calories = {}", &top3_sum);
    top3_sum
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::d01_calorie_counting::{count_calories_p1, find_top_three};

    #[test]
    fn test_part1() {
        let path = Path::new("../input-2022/d01-input.txt");
        let sums = count_calories_p1(path);
        assert_eq!(sums[0], 33480);
    }

    #[test]
    fn test_part2() {
        let path = Path::new("../input-2022/d01-input.txt");
        let mut sums:Vec<i32> = count_calories_p1(path);
        let sum = find_top_three(&mut sums);
        assert_eq!(sum, 207456);
    }
}