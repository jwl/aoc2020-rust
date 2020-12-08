use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|v| v.parse().expect("Failed to parse"))
        .collect()
}

#[aoc(day1, part1)]
fn find_2020_entries(input: &Vec<usize>) -> usize {
    let pair = input.iter().enumerate().find_map(|(idx, &item)| {
        match input
            .iter()
            .enumerate()
            .find(|(idx2, &second)| idx2 != &idx && (item + second == 2020))
        {
            Some((_, second)) => Some(item + second),
            None => None,
        }
    });
    pair.unwrap_or(1)
}

#[aoc(day1, part2)]
fn find_2020_entries_with_three(input: &Vec<usize>) -> usize {
    let result = input.iter().enumerate().find_map(|(idx, &item)| {
        input.iter().enumerate().find_map(|(idx2, &second)| {
            match input.iter().enumerate().find(|(idx3, &third)| {
                idx2 != idx && &idx != idx3 && &idx2 != idx3 && (item + second + third == 2020)
            }) {
                Some((_, third)) => Some(item * second * third),
                None => None,
            }
        })
    });
    result.unwrap_or(1)
}
