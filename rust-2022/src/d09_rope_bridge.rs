use std::io::{BufRead, BufReader};
use std::path::Path;

/// Advent of Code Day 09 - Rope Bride
///

/// Movement
#[derive(Debug)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

/// x,y position in a cartesian coordinate system
#[derive(Debug, Eq, PartialEq, Default, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

type Vector = Point;

impl Point {

    fn new(x: i32, y: i32) -> Self { Self { x, y } }

    /// compute the vector originating from this point to the other point
    fn vec_to(&self, other: &Point) -> Vector {
        Vector {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    /// returns the length of this vector rounded down to the nearest integer
    fn vec_length(&self) -> i32 {
        let len = ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt();
        len.trunc() as i32
    }

    /// returns a new point obtained from moving this point by some delta amount
    fn do_move(&self, delta: &Point) -> Point {
        Point {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }

    /// returns the delta vector required to move this point one unit towards the other point
    fn delta_one(&self, other: &Point) -> Point {
        let v = self.vec_to(other);
        match v {
            // other is on top of this point
            Vector { x: 0, y: 0 } => Point::default(),
            // other is directly up from this point
            Vector { x: 0, y: vy} if vy > 0 => Point::new(0, 1),
            // other is directly down from this point
            Vector { x: 0, y: vy} if vy < 0 => Point::new(0, -1),
            // other is directly left of this point
            Vector { x: vx, y: 0} if vx < 0 => Point::new(-1, 0),
            // other is directly right of this point
            Vector { x: vx, y: 0} if vx > 0 => Point::new(1, 0),
            // other is up and left from this point
            Vector { x: vx, y: vy } if vx < 0 && vy > 0 => Point::new(-1, 1),
            // other is up and right from this point
            Vector { x: vx, y: vy } if vx > 0 && vy > 0 => Point::new(1, 1),
            // other is down and left from this point
            Vector { x: vx, y: vy } if vx < 0 && vy < 0 => Point::new(-1, -1),
            // other os down and right
            Vector { x: vx, y: vy } if vx > 0 && vy < 0 => Point::new(1, -1),
            _ => panic!("{:?} did not match a pattern", &v),
        }
    }
}


/// read_input file into a vec of Moves
fn read_input(path: &Path) -> Vec<Move> {
    let f = std::fs::File::open(path).unwrap();
    let reader = BufReader::new(f);

    let moves : Vec<Move> = reader.lines()
        .map(|line| line.unwrap())
        .map(|s: String| {
            let v: Vec<&str> = s.split(" ").collect();
            let amt: i32 = v[1].parse().expect("valid i32 amount");
            match v[0] {
                "U" => Move::Up(amt),
                "D" => Move::Down(amt),
                "L" => Move::Left(amt),
                "R" => Move::Right(amt),
                _ => panic!("invalid move found in input"),
            }
        })
        .collect();

   moves
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::path::Path;
    use crate::d09_rope_bridge::{Move, Point, read_input, Vector};

    #[test]
    fn test_read_input() {
        let path = Path::new("../input-2022/d09-input.txt");
        let moves = read_input(&path);
        dbg!(moves);
    }

    #[test]
    fn do_part1() {
        let path = Path::new("../input-2022/d09-input.txt");
        let moves = read_input(&path);
        let mut visited: HashSet<Point> = HashSet::new();
        let mut head = Point::default();
        let mut tail = Point::default();
        visited.insert(tail);

        for mv in moves {
            // compute distance to move, and the amount delta to move the point one unit
            let (amt, delta) = {
                match mv {
                    Move::Up(amt) => (amt, Vector::new(0, 1)),
                    Move::Right(amt) => (amt, Vector::new(1, 0)),
                    Move::Down(amt) => (amt, Vector::new(0, -1)),
                    Move::Left(amt) => (amt, Vector::new(-1, 0)),
                }
            };
            for _ in 0..amt {
                // move head, and compute distance from tail to head
                head = head.do_move(&delta);
                if tail.vec_to(&head).vec_length() >= 2 {
                    // move tail one unit closer to head, and mark new pos as visited
                    let delta_vec = tail.delta_one(&head);
                    tail = tail.do_move(&delta_vec);
                    visited.insert(tail);
                }
            }
        }
        println!("tail visited {} positions at least once", visited.len())
    }

    #[test]
    fn do_part2() {
        let path = Path::new("../input-2022/d09-input.txt");
        let moves = read_input(&path);
        let mut points: Vec<Point> = Vec::with_capacity(10);
        for _ in 0..10 {
            points.push(Point::default());
        }
        let mut visited: HashSet<Point> = HashSet::new();
        visited.insert(Point::default());

        for mv in moves {
            let (amt, delta) = {
                match mv {
                    Move::Up(amt) => (amt, Vector::new(0, 1)),
                    Move::Right(amt) => (amt, Vector::new(1, 0)),
                    Move::Down(amt) => (amt, Vector::new(0, -1)),
                    Move::Left(amt) => (amt, Vector::new(-1, 0)),
                }
            };
            for _ in 0..amt {
                // move the head of points first
                // then iterate thru points 1..=9 to see if each of them should move based on
                //  the position of the previous point
                points[0] = points[0].do_move(&delta);
                for i in 1..=9_usize {
                    if points[i].vec_to(&points[i - 1]).vec_length() >= 2 {
                        let delta_vec = points[i].delta_one(&points[i - 1]);
                        points[i] = points[i].do_move(&delta_vec);
                        if i == 9 {
                            visited.insert(points[i]);
                        }
                    }
                }
            }
        }
        println!("part 2 tail visited {} positions at least once", visited.len())
    }
}