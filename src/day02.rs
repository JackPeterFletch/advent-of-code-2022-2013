use crate::util::read_input;
use std::collections::HashMap;
use RockPaperScissors::*;
use Outcome::*;
use once_cell::sync::Lazy;

static OPPOSITION_MOVE_ENCODING: Lazy<HashMap<char, RockPaperScissors>> = Lazy::new(|| HashMap::from([('A', Rock), ('B', Paper), ('C', Scissors)]));
static FRIENDLY_MOVE_ENCODING: Lazy<HashMap<char, RockPaperScissors>> = Lazy::new(|| HashMap::from([('X', Rock), ('Y', Paper), ('Z', Scissors)]));
static SUGGESTED_OUTCOME_ENCODING: Lazy<HashMap<char, Outcome>> = Lazy::new(|| HashMap::from([('X', Lose), ('Y', Draw), ('Z', Win)]));
static MOVE_SCORES: Lazy<HashMap<RockPaperScissors, usize>> = Lazy::new(|| HashMap::from([(Rock, 1), (Paper, 2), (Scissors, 3)]));

pub fn part1() {
    let suggestions: Vec<(char, char)> = read_suggestions();

    let results = suggestions.iter()
        .map(|encoded| (OPPOSITION_MOVE_ENCODING.get(&encoded.0).expect(""), FRIENDLY_MOVE_ENCODING.get(&encoded.1).expect("")))
        .map(|decoded| get_outcome_score(decoded.0, decoded.1) + MOVE_SCORES.get(decoded.1).expect(""));

    println!("{}", results.sum::<usize>())
}

pub fn part2() {
    let suggested_outcomes: Vec<(char, char)> = read_suggestions();

    let results = suggested_outcomes.iter()
        .map(|encoded| (OPPOSITION_MOVE_ENCODING.get(&encoded.0).expect(""), SUGGESTED_OUTCOME_ENCODING.get(&encoded.1).expect("")))
        .map(|decoded| (decoded.0, decide_move(decoded.0, decoded.1)))
        .map(|moves| get_outcome_score(moves.0, moves.1) + MOVE_SCORES.get(moves.1).expect(""));

    println!("{}", results.sum::<usize>())
}

fn read_suggestions() -> Vec<(char, char)> {
    read_input("input/day02")
        .iter()
        .map(|round| round.as_bytes())
        .map(|chars| (chars[0] as char, chars[2] as char)).collect()
}

fn get_outcome_score(opposition_move: &RockPaperScissors, friendly_move: &RockPaperScissors) -> usize {
    if friendly_move == opposition_move {
        3
    } else {
        match friendly_move {
            Rock => if *opposition_move == Scissors { 6 } else { 0 }
            Paper => if *opposition_move == Rock { 6 } else { 0 }
            Scissors => if *opposition_move == Paper { 6 } else { 0 }
        }
    }
}

fn decide_move<'a>(opposition_move: &'a RockPaperScissors, suggested_outcome: &Outcome) -> &'a RockPaperScissors {
    match suggested_outcome {
        Draw => opposition_move,
        Win => match opposition_move {
            Rock => &Paper,
            Paper => &Scissors,
            Scissors => &Rock
        },
        Lose => match opposition_move {
            Rock => &Scissors,
            Paper => &Rock,
            Scissors => &Paper
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Outcome {
    Lose,
    Win,
    Draw
}