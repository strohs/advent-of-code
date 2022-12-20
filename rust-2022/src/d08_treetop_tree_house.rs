use std::io::{BufRead, BufReader};
use std::path::Path;

/// Day 08 Tree Top TreeHouse
///

type Matrix = Vec<Vec<u8>>;


/// read_input file into a Matrix of u8 digits
fn read_input(path: &Path) -> Matrix {
    let f = std::fs::File::open(path).unwrap();
    let reader = BufReader::new(f);

    let matrix : Matrix = reader.lines()
        .map(|l| l.unwrap())
        .map(|s|
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
        .collect();

    matrix
}

/// returns true if the element (tree height) at the given row, col in the Matrix is visible from the edge
fn is_visible(matrix: &Matrix, row: usize, col: usize) -> bool {

    let tree_height = matrix[row][col];

    // check from the North of the given position
    let from_north = (0..row).all(|r| matrix[r][col] < tree_height);
    let from_south = (row+1..matrix.len()).all(|r| matrix[r][col] < tree_height);
    let from_west = (0..col).all(|c| matrix[row][c] < tree_height);
    let from_east = (col+1..matrix[0].len()).all(|c| matrix[row][c] < tree_height);

    from_north || from_south || from_west || from_east
}

/// compute the scenic score for a tree height at row,col in the matrix
fn scenic_score(matrix: &Matrix, row: usize, col: usize) -> usize {
    // target tree height being examined
    let tt = matrix[row][col];

    // computes the count of trees <= tt, stopping once the first tree == tt is found
    let tree_count = |heights: &[u8]|
        heights.iter()
            .position(|h| *h == tt)
            .map_or(heights.len(), |pos| pos + 1);


    let north_heights: Vec<u8> = matrix[..row].iter().rev().map(|v| v[col]).take_while(|h| *h <= tt).collect();
    let south_heights: Vec<u8> = matrix[row+1..].iter().map(|v| v[col]).take_while(|h| *h <= tt).collect();
    let west_heights: Vec<u8> = matrix[row][..col].iter().rev().take_while(|h| **h <= tt).cloned().collect();
    let east_heights: Vec<u8> = matrix[row][col+1..].iter().take_while(|h| **h <= tt).cloned().collect();

    tree_count(&north_heights) * tree_count(&south_heights) * tree_count(&west_heights) * tree_count(&east_heights)
}

/// how many trees are visible from outside the grid?
fn part1(input_path: &Path) {
    let matrix = read_input(input_path);
    let mut count: usize = 0;
    // dont iterate the outside edge of the matrix, start from the first inner row/col
    for r in 1..(matrix.len()-1) {
        for c in 1..(matrix[0].len() - 1) {
            if is_visible(&matrix, r, c) {
                count += 1;
                //println!("{:02},{:02} : {} ", r, c, &matrix[r][c]);
            }
        }
    }
    // all trees on the perimeter are visible
    let perim_length = (matrix[0].len() * 2) + ((matrix.len() - 2) * 2);
    count += perim_length;

    println!("There are {count} trees visible from outside the grid");
}

fn part2(input_path: &Path) {
    let matrix = read_input(input_path);

    let mut highest_score = usize::MIN;

    // dont iterate the outside edge of the matrix, start from the first inner row/col
    for r in 1..(matrix.len()-1) {
        for c in 1..(matrix[0].len() - 1) {
            let score = scenic_score(&matrix, r, c);
            println!("score {r},{c}  = {score}");
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("highest scenic score is {}", highest_score);
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::d08_treetop_tree_house::{is_visible, Matrix, part1, part2, read_input};

    #[test]
    fn do_part1() {
        part1(Path::new("../input-2022/d08-input.txt"));
    }

    #[test]
    fn do_part2() {
        part2(Path::new("../input-2022/d08-input.txt"));
    }

    #[test]
    fn test_parse() {
        let mats = read_input(Path::new("../input-2022/d08-input.txt"));
        assert_eq!(mats[0].len(), 99);
        assert_eq!(mats.len(), 99);

    }

    #[test]
    fn test_is_visibile() {
        let matrix: Matrix = vec![
            vec![4, 3, 2],
            vec![0, 4, 3],
            vec![0, 0, 4],
        ];
        assert!(is_visible(&matrix, 1, 1));
    }

    #[test]
    fn test_is_not_visibile() {
        let matrix: Matrix = vec![
            vec![4, 3, 2],
            vec![5, 2, 3],
            vec![0, 4, 4],
        ];
        assert!(!is_visible(&matrix, 1, 1));
    }

    #[test]
    fn test_perim_length() {
        let matrix: Matrix = vec![
            vec![4, 3, 2],
            vec![5, 2, 3],
            vec![0, 4, 4],
        ];
        let len = (matrix[0].len() * 2) + ((matrix.len() - 2) * 2);
        assert_eq!(len, 8);
    }

    #[test]
    fn test_north_trees() {
        let matrix: Matrix = vec![
            vec![ 1,  2,  3,  4],
            vec![ 5,  6,  7,  8],
            vec![ 1,  4,  3,  5],
            vec![ 1,  5,  7 , 8],
        ];
        let row = 1;
        let col = 1;
        let tt = matrix[1][1];
        let north_heights: Vec<u8> = matrix[..row].iter().rev().map(|v| v[col]).take_while(|h| *h <= tt).collect();
        let south_heights: Vec<u8> = matrix[row+1..].iter().map(|v| v[col]).take_while(|h| *h <= tt).collect();
        let west_heights: Vec<u8> = matrix[row][..col].iter().rev().take_while(|h| **h <= tt).cloned().collect();
        let east_heights: Vec<u8> = matrix[row][col+1..].iter().take_while(|h| **h <= tt).cloned().collect();
        assert_eq!(north_heights, vec![2]);
        assert_eq!(south_heights, vec![4, 5]);
        assert_eq!(east_heights, vec![]);
        assert_eq!(west_heights, vec![5]);
    }
}