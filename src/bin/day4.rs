use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day4.txt")?;

    let mut winning_nos = Vec::new();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut winning_set;

    for line in input.lines() {
        let (_, line) = line.split_once(':').unwrap();
        let (winning, ticket) = line.split_once('|').unwrap();
        winning_set = winning.split_whitespace().collect::<HashSet<_>>();

        let no_winning = ticket
            .split_whitespace()
            .filter(|number| winning_set.contains(number))
            .count();

        let copies = winning_nos
            .iter()
            .rev()
            .take(25)
            .enumerate()
            .filter_map(|(j, &(no_winning, copies))| (no_winning > j).then_some(copies))
            .sum::<usize>()
            + 1usize;

        winning_nos.push((no_winning, copies));

        part1 += no_winning
            .checked_sub(1)
            .map(|x| 2u32.pow(x as u32))
            .unwrap_or(0);
        part2 += copies;
    }

    println!("4.1: {part1}");
    println!("4.2: {part2}");
    Ok(())
}
