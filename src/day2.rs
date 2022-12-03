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

    fn get_correct_move(opp_move: &Move, desired_state: &MatchState) -> Move {
        match desired_state {
            MatchState::Win => opp_move.get_counter(),
            MatchState::Lose => opp_move.get_loser(),
            MatchState::Draw => *opp_move,
        }
    }
}
enum MatchState {
    Win,
    Lose,
    Draw,
}

impl MatchState {
    fn char_to_state(char: &str) -> MatchState {
        match char {
            "X" => MatchState::Lose,
            "Y" => MatchState::Draw,
            "Z" => MatchState::Win,
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
            &Round(Rock, Paper) => 6,
            &Round(Paper, Scissors) => 6,
            &Round(Scissors, Rock) => 6,
            _ => 0,
        }
    }

    fn point_for_rounds(&self) -> u32 {
        let mut total = 0;
        total += Round::round_condition_points(self);

        total += match self.1 {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        return total;
    }
}

fn line_to_round(line: &str) -> Round {
    let moves: Vec<&str> = line.split(" ").collect();

    let first_move = Move::char_to_move(moves[0]);
    let desired_state = MatchState::char_to_state(moves[1]);
    let second_move = Move::get_correct_move(&first_move, &desired_state);

    return Round(first_move, second_move);
}

// This doesn't work correctly after implementing the second part 
// (expected, but idk if it's acceptable)
pub fn part_1(file: &String) -> u32 {
    file.split("\n")
        .map(line_to_round)
        .map(|round| round.point_for_rounds())
        .sum()
}

pub fn part_2(file: &String) -> u32 {
    file.split("\n")
        .map(line_to_round)
        .map(|round| round.point_for_rounds())
        .sum()
}
