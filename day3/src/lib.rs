use std::collections::HashMap;
use itertools::Itertools;

pub fn process_part1(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result: usize = input
        .lines()
        .map(|rucksack| {
            let sack_len = rucksack.len() / 2;
            let first_compartment: &str = &rucksack[0..sack_len];
            let second_compartment: &str = &rucksack[sack_len..(sack_len * 2)];

            let common_char = first_compartment
                .chars()
                .find(|c| second_compartment.contains(*c))
                .unwrap();

            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();
    
    let result: usize = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let bags: Vec<&str> = chunk.collect();

            let common_char = bags[0]
                .chars()
                .find(|c| {
                    bags[1].contains(*c)
                        && bags[2].contains(*c)
                })
                .unwrap();
                
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}
