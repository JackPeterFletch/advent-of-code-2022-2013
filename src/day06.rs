use std::collections::HashSet;
use crate::util::read_input;

pub fn part1() -> usize {
    find_distinct_window(4)
}

pub fn part2() -> usize {
    find_distinct_window(14)
}

fn find_distinct_window(window_size: usize) -> usize {
    let input_iterator: Vec<char> = read_input("input/day06")[0]
        .chars()
        .collect();

    let mut starting_pos = 0;
    for (pos, n) in input_iterator.windows(window_size).enumerate() {
        let mut set: HashSet<char> = HashSet::new();

        for i in 0..window_size {
            set.insert(n[i]);
        }

        if set.len() == window_size {
            starting_pos = pos + window_size;
            break;
        }
    }
    starting_pos
}