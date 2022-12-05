#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn char_to_move(char: &str) -> Move {
        match char {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => unreachable!("This is not a valid input"),
        }
    }

    fn get_counter(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn get_loser(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn get_correct_move(opp_move: &Move, desired_state: &RoundState) -> Move {
        match desired_state {
            RoundState::Win => opp_move.get_counter(),
            RoundState::Lose => opp_move.get_loser(),
            RoundState::Draw => *opp_move,
        }
    }
}
enum RoundState {
    Win,
    Lose,
    Draw,
}

impl RoundState {
    fn char_to_state(char: &str) -> RoundState {
        match char {
            "X" => RoundState::Lose,
            "Y" => RoundState::Draw,
            "Z" => RoundState::Win,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Round(Move, Move);

impl Round {
    fn round_condition_points(round: &Self) -> u32 {
        use Move::{Paper, Rock, Scissors};

        if round.0 == round.1 {
            return 3;
        }

        match round {
            &Round(Rock, Paper) | &Round(Paper, Scissors) | &Round(Scissors, Rock) => 6,
            _ => 0,
        }
    }

    fn point_for_round(&self) -> u32 {
        Round::round_condition_points(self)
            + match self.1 {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
            }
    }
}

fn line_to_round(line: &str) -> Round {
    let moves: Vec<&str> = line.split(' ').collect();

    let first_move = Move::char_to_move(moves[0]);
    let desired_state = RoundState::char_to_state(moves[1]);
    let second_move = Move::get_correct_move(&first_move, &desired_state);

    Round(first_move, second_move)
}

// This doesn't work correctly after implementing the second part
// (expected, but idk if it's acceptable)
pub fn part_1(file: &str) -> u32 {
    file.split('\n')
        .map(line_to_round)
        .map(|round| round.point_for_round())
        .sum()
}

pub fn part_2(file: &str) -> u32 {
    file.split('\n')
        .map(line_to_round)
        .map(|round| round.point_for_round())
        .sum()
}
