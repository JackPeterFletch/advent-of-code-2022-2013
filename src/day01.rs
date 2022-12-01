use crate::util::read_input;
use std::cmp::max;

pub fn run2() {
    let input = read_input("input/day1");

    let mut highest: usize = 0;
    let mut current: usize = 0;

    for line in input {
        if line.trim().is_empty() {
            if current > highest {
                highest = current;
                current = 0;
            } else {
                current = 0;
            }
        } else {
            current = current + line.parse::<usize>().expect("Cannot Parse Int");
        }
    }

    println!("{}", highest)
}

pub fn run1() {
    let result = read_input("input/day1")
        .split(|v| v.trim().is_empty())
        .map(|snacks| snacks.iter().map(|calories| calories.parse::<usize>().expect("Cannot Parse Int")))
        .fold(0, | highestTotalCalories , snacks| max(snacks.sum(), highestTotalCalories));

    println!("{}", result)
}