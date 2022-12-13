use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
enum Errors {
    #[error("Is not a valid move")]
    InvalidMove,
    #[error("Is not a valid round")]
    InvalidRound,
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
impl Move {
    fn calc_choice_points(&self) -> u16 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn calc_move_from_opponent(res: char, theirs: Move) -> Result<Self, Errors> {
        match (res, theirs) {
            ('X', Move::Rock) => Ok(Self::Scissors),
            ('X', Move::Paper) => Ok(Self::Rock),
            ('X', Move::Scissors) => Ok(Self::Paper),
            ('Y', _) => Ok(theirs.clone()),
            ('Z', Move::Rock) => Ok(Self::Paper),
            ('Z', Move::Paper) => Ok(Self::Scissors),
            ('Z', Move::Scissors) => Ok(Self::Rock),
            (_, _) => Err(Errors::InvalidMove),
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn calc_outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            return Outcome::Win;
        } else if theirs.beats(self) {
            return Outcome::Lose;
        } else {
            return Outcome::Draw;
        }
    }
}
impl TryFrom<char> for Move {
    type Error = Errors;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(Self::Error::InvalidMove),
        }
    }
}

enum Outcome {
    Draw,
    Win,
    Lose,
}
impl Outcome {
    fn calc_outcome_points(self) -> u16 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug)]
struct Round {
    theirs: Move,
    ours: Move,
}
impl Round {
    fn calc_score(&self) -> u16 {
        let choice_score = self.ours.calc_choice_points();
        let outcome_score = Outcome::calc_outcome_points(self.ours.calc_outcome(self.theirs));

        return choice_score + outcome_score;
    }
}
impl FromStr for Round {
    type Err = Errors;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = value.chars().filter(|&c| c != ' ').collect();

        let (theirs, ours) = (chars[0], chars[1]) else {
            return Err(Self::Err::InvalidMove);
        };

        let theirs: Move = theirs.try_into()?;

        Ok(Self {
            theirs,
            ours: Move::calc_move_from_opponent(ours, theirs)?, // part TWO
                                                                // ours: ours.try_into()?, // part ONE
        })
    }
}

fn main() {
    let moves = include_str!("strategy.txt");
    let total_scores: u16 = moves
        .lines()
        .map(|line| line.parse::<Round>())
        .map(|round| round.expect("exepected round").calc_score())
        .sum();

    println!("Total score = {:?}", total_scores);
}
