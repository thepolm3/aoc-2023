use std::{collections::HashSet, io::empty};

use anyhow::Context;
use itertools::Itertools;

fn d1d(x1: usize, x2: usize, expands: &HashSet<usize>, factor: usize) -> usize {
    if x1 > x2 {
        return d1d(x2, x1, expands, factor);
    }
    expands.iter().filter(|&x| x1 < *x && *x < x2).count() * (factor - 1) + x2 - x1
}
fn d(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    expanding_rows: &HashSet<usize>,
    expanding_columns: &HashSet<usize>,
    factor: usize,
) -> usize {
    d1d(x1, x2, expanding_columns, factor) + d1d(y1, y2, expanding_rows, factor)
}
fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day11.txt")?;
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut galaxies = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert((x, y));
            }
        }
    }
    let empty_columns: HashSet<_> = (0..width)
        .filter(|x| !galaxies.iter().any(|(gx, _)| gx == x))
        .collect();
    let empty_rows: HashSet<_> = (0..height)
        .filter(|y| !galaxies.iter().any(|(_, gy)| gy == y))
        .collect();

    let (part1, part2) = galaxies
        .iter()
        .combinations(2)
        .map(|v| {
            (
                d(*v[0], *v[1], &empty_rows, &empty_columns, 2),
                d(*v[0], *v[1], &empty_rows, &empty_columns, 1_000_000),
            )
        })
        .reduce(|(a, b), (x, y)| (a + x, b + y))
        .context("No galaxies")?;
    // println!("{}", d((1, 5), (4, 9), &empty_rows, &empty_columns));
    println!("11.1: {part1}");
    println!("11.2: {part2}");

    Ok(())
}
