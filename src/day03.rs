use crate::util::read_input;
use std::collections::{HashMap, HashSet};
use once_cell::sync::Lazy;

pub fn part1() -> usize {
    read_rucksack()
        .iter()
        .map(|pouch| pouch.split_at(pouch.len()/2))
        .map(|it| get_common_character(it.0.to_vec(), it.1.to_vec()))
        .map(|it| PRIORITIES[&it])
        .sum()
}

pub fn part2() -> usize {
    read_rucksack()
        .chunks(3)
        .map(|group| get_common_character_for_group(&group[0], &group[1], &group[2]))
        .map(|it| PRIORITIES[&it])
        .sum()
}

fn get_common_character_for_group(first: &Vec<char>, second: &Vec<char>, third: &Vec<char>) -> char {
    let set1: HashSet<&char> = HashSet::from_iter(first.iter());
    let set2: HashSet<&char> = HashSet::from_iter(second.iter());

    let first_common = third.iter().find(|c| set1.contains(c) && set2.contains(c)).expect("Common char not found");
    return first_common.clone()
}

fn get_common_character(left: Vec<char>, right: Vec<char>) -> char {
    let set: HashSet<&char> = HashSet::from_iter(left.iter());

    let first_common = right.iter().find(|c| set.contains(c)).expect("Common char not found");
    return first_common.clone()
}

fn read_rucksack() -> Vec<Vec<char>> {
    read_input("input/day03")
        .iter()
        .map(|it| it.chars().collect())
        .collect()
}

const PRIORITIES: Lazy<HashMap<char, usize>> = Lazy::new(|| HashMap::from([
    ('a', 1),
    ('b', 2),
    ('c', 3),
    ('d', 4),
    ('e', 5),
    ('f', 6),
    ('g', 7),
    ('h', 8),
    ('i', 9),
    ('j', 10),
    ('k', 11),
    ('l', 12),
    ('m', 13),
    ('n', 14),
    ('o', 15),
    ('p', 16),
    ('q', 17),
    ('r', 18),
    ('s', 19),
    ('t', 20),
    ('u', 21),
    ('v', 22),
    ('w', 23),
    ('x', 24),
    ('y', 25),
    ('z', 26),

    ('A', 27),
    ('B', 28),
    ('C', 29),
    ('D', 30),
    ('E', 31),
    ('F', 32),
    ('G', 33),
    ('H', 34),
    ('I', 35),
    ('J', 36),
    ('K', 37),
    ('L', 38),
    ('M', 39),
    ('N', 40),
    ('O', 41),
    ('P', 42),
    ('Q', 43),
    ('R', 44),
    ('S', 45),
    ('T', 46),
    ('U', 47),
    ('V', 48),
    ('W', 49),
    ('X', 50),
    ('Y', 51),
    ('Z', 52),
]));