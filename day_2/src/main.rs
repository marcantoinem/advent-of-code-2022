use std::fs;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
enum MatchResult {
    Win,
    Lose,
    Equal,
}

impl Choice {
    fn to_value(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        }
    }
    fn compare_with_player(&self, player1: &Self) -> MatchResult {
        match (self, player1) {
            (player1, player2) if player1 == player2 => MatchResult::Equal,
            (Choice::Rock, Choice::Scissor)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissor, Choice::Paper) => MatchResult::Win,
            (_, _) => MatchResult::Lose,
        }
    }
    fn obtain_result(&self, result: &MatchResult) -> Self {
        match (result, self) {
            (MatchResult::Equal, choice) => *choice,
            (MatchResult::Lose, Choice::Paper) => Choice::Rock,
            (MatchResult::Win, Choice::Scissor) => Choice::Rock,
            (MatchResult::Lose, Choice::Scissor) => Choice::Paper,
            (MatchResult::Win, Choice::Rock) => Choice::Paper,
            (MatchResult::Lose, Choice::Rock) => Choice::Scissor,
            (MatchResult::Win, Choice::Paper) => Choice::Scissor,
        }
    }
}

impl MatchResult {
    fn to_value(&self) -> usize {
        match self {
            MatchResult::Win => 6,
            MatchResult::Equal => 3,
            MatchResult::Lose => 0,
        }
    }
}

fn score(text: &str) -> usize {
    let first_choice = match &text[0..1] {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissor,
        _ => unreachable!(),
    };
    let our_choice = match &text[2..3] {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissor,
        _ => unreachable!(),
    };
    our_choice.to_value() + our_choice.compare_with_player(&first_choice).to_value()
}

fn score_part2(text: &str) -> usize {
    let first_choice = match &text[0..1] {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissor,
        _ => unreachable!(),
    };
    let match_result = match &text[2..3] {
        "X" => MatchResult::Lose,
        "Y" => MatchResult::Equal,
        "Z" => MatchResult::Win,
        _ => unreachable!(),
    };
    let our_choice = first_choice.obtain_result(&match_result);
    println!("{:?} {:?} {:?}", first_choice, match_result, our_choice);
    our_choice.to_value() + our_choice.compare_with_player(&first_choice).to_value()
}

fn main() {
    let result = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(score_part2)
        .sum::<usize>();
    println!("{}", result);
}
