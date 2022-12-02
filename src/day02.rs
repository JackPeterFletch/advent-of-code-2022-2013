use crate::util::read_input;
use std::collections::HashMap;
use RockPaperScissors::*;
use Outcome::*;
use once_cell::sync::Lazy;
use unwrap::unwrap;

static OPPOSITION_MOVE_ENCODING: Lazy<HashMap<char, RockPaperScissors>> = Lazy::new(|| HashMap::from([('A', Rock), ('B', Paper), ('C', Scissors)]));
static FRIENDLY_MOVE_ENCODING: Lazy<HashMap<char, RockPaperScissors>> = Lazy::new(|| HashMap::from([('X', Rock), ('Y', Paper), ('Z', Scissors)]));
static SUGGESTED_OUTCOME_ENCODING: Lazy<HashMap<char, Outcome>> = Lazy::new(|| HashMap::from([('X', Lose), ('Y', Draw), ('Z', Win)]));
static MOVE_SCORES: Lazy<HashMap<RockPaperScissors, usize>> = Lazy::new(|| HashMap::from([(Rock, 1), (Paper, 2), (Scissors, 3)]));

pub fn part1() -> usize {
    read_suggestions().iter()
        .map(|it| (unwrap!(OPPOSITION_MOVE_ENCODING.get(&it.0)), unwrap!(FRIENDLY_MOVE_ENCODING.get(&it.1))))
        .map(|it| get_outcome_score(it.0, it.1) + unwrap!(MOVE_SCORES.get(it.1)))
        .sum::<usize>()
}

pub fn part2() -> usize {
    read_suggestions().iter()
        .map(|it| (unwrap!(OPPOSITION_MOVE_ENCODING.get(&it.0)), unwrap!(SUGGESTED_OUTCOME_ENCODING.get(&it.1))))
        .map(|it| (it.0, decide_move(it.0, it.1)))
        .map(|it| get_outcome_score(it.0, it.1) + unwrap!(MOVE_SCORES.get(it.1)))
        .sum::<usize>()
}

fn read_suggestions() -> Vec<(char, char)> {
    read_input("input/day02")
        .iter()
        .map(|it| it.as_bytes())
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