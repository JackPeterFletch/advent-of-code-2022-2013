use std::collections::{HashMap, VecDeque};
use crate::util::read_input;

const INPUT: &str = "input/day05";

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Move {
    number: usize,
    from: char,
    to: char
}

pub fn part1() -> String {
    let mut stacks = get_stacks();

    for mov in &get_moves() {
        crate_mover_9000(mov, &mut stacks)
    }

    get_result(&mut stacks)
}

pub fn part2() -> String {
    let mut stacks = get_stacks();

    for mov in &get_moves() {
        crate_mover_9001(mov, &mut stacks)
    }

    get_result(&mut stacks)
}

fn crate_mover_9001(mov: &Move, stacks: &mut HashMap<String, VecDeque<char>>) {
    let mut tmp: VecDeque<char> =  VecDeque::new();

    let from = stacks.get_mut(&mov.from.to_string()).unwrap();
    for _ in 0..mov.number {
        tmp.push_front(from.pop_front().expect(""));

    }

    let to = stacks.get_mut(&mov.to.to_string()).unwrap();
    for _ in 0..mov.number {
        to.push_front(tmp.pop_front().expect(""));
    }
}

fn crate_mover_9000(mov: &Move, stacks: &mut HashMap<String, VecDeque<char>>) {
    for _ in 0..mov.number {
        let from = stacks.get_mut(&mov.from.to_string()).unwrap();
        let tmp = from.pop_front().unwrap();
        let to = stacks.get_mut(&mov.to.to_string()).unwrap();
        to.push_front(tmp);
    }
}

fn get_result(stacks: &mut HashMap<String, VecDeque<char>>) -> String {
    let mut result = String::new();
    for n in 1..(get_no_stacks() + 1) {
        result = result.to_owned() + &stacks.get_mut(&n.to_string()).unwrap().pop_front().expect("").to_string();
    }
    result
}

fn get_moves() -> Vec<Move> {
    read_input(INPUT)
        .iter()
        .filter(|it| it.contains("move"))
        .map(|it| it.split(" ").collect::<Vec<&str>>())
        .map(|it| Move {
            number: it.get(1).expect("").parse::<usize>().expect(""),
            from: it.get(3).expect("").parse().unwrap(),
            to: it.get(5).expect("").parse().unwrap()
        })
        .collect()
}

fn get_stacks() -> HashMap<String,VecDeque<char>> {
    let mut stacks: HashMap<String, VecDeque<char>> = HashMap::new();

    for n in 0..get_no_stacks() {
        stacks.insert((n+1).to_string(), VecDeque::new());
    }

    fill_stacks(&mut stacks);

    stacks
}

fn fill_stacks(stacks: &mut HashMap<String, VecDeque<char>>) {
    let stack_input: Vec<Vec<char>> = read_input(INPUT)
        .iter()
        .filter(|it| it.contains('['))
        .map(|it| it.chars().collect())
        .collect();

    for stack_input_row in stack_input {
        for n in 0..stacks.len() {
            let char = stack_input_row.get(get_stack_index(n));

            if char.is_some() && !char.expect("").is_ascii_whitespace() {
                stacks.get_mut(&*(n + 1).to_string()).unwrap().push_back(*char.expect(""))

            }
        }
    }
}

fn get_stack_index(number: usize) -> usize {
    (number * 4) + 1
}

fn get_no_stacks() -> usize {
    read_input(INPUT)
        .iter()
        .filter(|it| it.contains(" 1   2"))
        .last().expect("Could not find Crate list")
        .trim()
        .split_ascii_whitespace()
        .count()
}