use crate::util::read_input;
use std::cmp::max;

pub fn part1_2() -> usize {
    let input = read_input("input/day01");

    let mut highest: usize = 0;
    let mut current: usize = 0;

    for line in input {
        if line.trim().is_empty() {
            if current > highest {
                highest = current;
            }
            current = 0;
        } else {
            current += line.parse::<usize>().expect("Cannot Parse Int");
        }
    }

    highest
}

pub fn part1_1() -> usize {
    read_input("input/day01")
        .split(|v| v.trim().is_empty())
        .map(|snacks| snacks.iter().map(|it|it.parse::<usize>().expect("Cannot Parse Int")))
        .fold(0, |highest_total_calories, snacks| max(snacks.sum(), highest_total_calories))
}


pub fn part2() -> usize {
    let mut results: Vec<usize> = read_input("input/day01")
        .split(|v| v.trim().is_empty())
        .map(|snacks| snacks.iter().map(|it|it.parse::<usize>().expect("Cannot Parse Int")).collect::<Vec<usize>>().iter().sum())
        .collect();

    results.sort();

    results.pop().expect("") + results.pop().expect("") + results.pop().expect("")
}