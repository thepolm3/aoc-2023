use std::collections::BTreeSet;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day11.txt")?;

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut galaxies = BTreeSet::new();
    let mut gcount = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                galaxies.insert((x, y));
                gcount += 1;
            }
        }
    }

    let empty_column_contributions: usize = (0..width)
        .filter(|x| !galaxies.iter().any(|(gx, _)| gx == x))
        .map(|x| {
            //since BTreeSet is sorted, galaxies will be in order so we look for the first galaxy
            let left = galaxies.iter().position(|(gx, _)| *gx >= x).unwrap();
            left * (gcount - left)
        })
        .sum();

    let empty_row_contributions: usize = (0..height)
        .filter(|y| !galaxies.iter().any(|(_, gy)| gy == y))
        .map(|y| {
            //BTreeSet is not sorted for y accesses
            let above = galaxies.iter().filter(|(_, gy)| *gy < y).count();
            above * (gcount - above)
        })
        .sum();

    let normal_distance: usize = galaxies
        .iter()
        .combinations(2)
        .map(|v| v[1].0.abs_diff(v[0].0) + v[1].1.abs_diff(v[0].1))
        .sum();

    let expansion_factor = empty_row_contributions + empty_column_contributions;
    // println!("{}", d((1, 5), (4, 9), &empty_rows, &empty_columns));
    println!(
        "11.1: {}",
        normal_distance + empty_column_contributions + empty_row_contributions
    );
    println!(
        "11.2: {}",
        normal_distance + expansion_factor * (1_000_000 - 1)
    );

    Ok(())
}
