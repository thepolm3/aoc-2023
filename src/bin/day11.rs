use std::collections::HashSet;

use itertools::Itertools;
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
    let empty_column_contributions: usize = (0..width)
        .filter(|x| !galaxies.iter().any(|(gx, _)| gx == x))
        .map(|x| {
            let left = galaxies.iter().filter(|(gx, _)| *gx < x).count();
            let right = galaxies.iter().filter(|(gx, _)| *gx > x).count();
            left * right
        })
        .sum();

    let empty_row_contributions: usize = (0..height)
        .filter(|y| !galaxies.iter().any(|(_, gy)| gy == y))
        .map(|y| {
            let above = galaxies.iter().filter(|(_, gy)| *gy < y).count();
            let below = galaxies.iter().filter(|(_, gy)| *gy > y).count();
            above * below
        })
        .sum();

    let normal_distance: usize = galaxies
        .iter()
        .combinations(2)
        .map(|v| v[1].0.abs_diff(v[0].0) + v[1].1.abs_diff(v[0].1))
        .sum();
    // println!("{}", d((1, 5), (4, 9), &empty_rows, &empty_columns));
    println!(
        "11.1: {}",
        normal_distance + empty_column_contributions + empty_row_contributions
    );
    println!(
        "11.2: {}",
        normal_distance + (empty_column_contributions + empty_row_contributions) * (1_000_000 - 1)
    );

    Ok(())
}
