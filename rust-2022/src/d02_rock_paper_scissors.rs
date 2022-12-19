use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Loss,
    Draw,
    Win,
}
impl Outcome {
    pub const fn score(&self) -> i32 {
        match *self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}
impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("invalid input string for Outcome: {}", &s),
        }
    }
}



#[derive(Debug, Eq, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub const fn score(&self) -> i32 {
        match *self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("invalid input string for Hand: {}", &s),
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Hand::Rock => f.write_str("ROCK"),
            Hand::Paper => f.write_str("PAPER"),
            Hand::Scissors => f.write_str("SCISSORS"),
        }
    }
}

/// compute the score for a round of rock paper scissors
fn round_score(opponent: &Hand, player: &Hand) -> i32 {
    let round_score = match (opponent, player) {
        (Hand::Rock, Hand::Paper) => Outcome::Win.score(),
        (Hand::Rock, Hand::Scissors) => Outcome::Loss.score(),
        (Hand::Paper, Hand::Rock) => Outcome::Loss.score(),
        (Hand::Paper, Hand::Scissors) => Outcome::Win.score(),
        (Hand::Scissors, Hand::Rock) => Outcome::Win.score(),
        (Hand::Scissors, Hand::Paper) => Outcome::Loss.score(),
        _ => Outcome::Draw.score()
    };
    round_score + player.score()
}

/// determine what hand you should play given your opponents hand and a desired outcome
fn determine_hand(opponent: &Hand, outcome: &Outcome) -> Hand {
    match (opponent, outcome) {
        (Hand::Rock, Outcome::Loss) => Hand::Scissors,
        (Hand::Rock, Outcome::Draw) => Hand::Rock,
        (Hand::Rock, Outcome::Win) => Hand::Paper,
        (Hand::Paper, Outcome::Loss) => Hand::Rock,
        (Hand::Paper, Outcome::Draw) => Hand::Paper,
        (Hand::Paper, Outcome::Win) => Hand::Scissors,
        (Hand::Scissors, Outcome::Loss) => Hand::Paper,
        (Hand::Scissors, Outcome::Draw) => Hand::Scissors,
        (Hand::Scissors, Outcome::Win) => Hand::Rock,
    }
}

fn total_score_part1(file: &Path) -> i32 {
    let f = File::open(file).expect("input file can be found");
    let reader = BufReader::new(f);

    let mut total_score: i32 = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            let mut split = l.split_ascii_whitespace();
            let opponent = Hand::from(split.next().expect("valid input line"));
            let player = Hand::from(split.next().expect("valid input line"));
            total_score += round_score(&opponent, &player);
        }
    }

    total_score
}

fn total_score_part2(file: &Path) -> i32 {
    let f = File::open(file).expect("input file can be found");
    let reader = BufReader::new(f);

    let mut total_score: i32 = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            let mut split = l.split_ascii_whitespace();
            let opponent = Hand::from(split.next().expect("valid input line"));
            let outcome = Outcome::from(split.next().expect("valid input line"));
            total_score += round_score(&opponent, &determine_hand(&opponent, &outcome));
        }
    }

    total_score
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::d02_rock_paper_scissors::{determine_hand, Hand, Outcome, total_score_part1, total_score_part2};

    #[test]
    fn determine_hand_draw() {
        let op = Hand::Rock;
        let out = Outcome::Draw;
        assert_eq!(determine_hand(&op, &out), Hand::Rock);
    }

    #[test]
    fn determine_hand_win() {
        let op = Hand::Rock;
        let out = Outcome::Win;
        assert_eq!(determine_hand(&op, &out), Hand::Paper);
    }

    #[test]
    fn determine_hand_loss() {
        let op = Hand::Rock;
        let out = Outcome::Loss;
        assert_eq!(determine_hand(&op, &out), Hand::Scissors);
    }

    #[test]
    fn test_part1() {
        let path = Path::new("../input/d02-input.txt");
        let total = total_score_part1(path);
        println!("total score for strat guide is {total}");
        assert_eq!(total, 12156);
    }

    #[test]
    fn test_part2() {
        let path = Path::new("../input/d02-input.txt");
        let total = total_score_part2(path);
        println!("total score for part 2 is {total}");
        assert_eq!(total, 10835);
    }
}