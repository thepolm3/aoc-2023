use std::collections::{BTreeMap, BTreeSet, HashSet};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Rock,
    Plot,
}
impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Cell::Rock,
            '.' | 'S' => Cell::Plot,
            _ => panic!("Invalid cell!"),
        }
    }
}

struct Garden {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}
impl Garden {
    fn from_str(input: &str) -> Option<Self> {
        let lines: Vec<_> = input.lines().collect();
        let width = lines.first()?.len();
        let height = lines.len();

        let cells = lines
            .into_iter()
            .flat_map(|line| line.chars().map(Cell::from_char))
            .collect();

        Some(Self {
            cells,
            width,
            height,
        })
    }

    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get(y * self.width + x).copied()
    }
}

fn parity((x, y): &(usize, usize)) -> bool {
    ((x + y) % 2) == 0
}

fn main() {
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    // let input = std::fs::read_to_string("inputs/day21.txt").unwrap();
    let start = if let (start_y, Some(start_x)) = input
        .lines()
        .map(|line| line.chars().position(|c| c == 'S'))
        .find_position(Option::is_some)
        .unwrap()
    {
        (start_x, start_y)
    } else {
        panic!("No start")
    };
    let garden = Garden::from_str(&input).unwrap();

    let mut visited = BTreeSet::new();
    let mut hull = BTreeSet::new();
    let mut next_hull = BTreeSet::new();
    hull.insert(start);

    let loops = 64;
    for _ in 0..loops {
        while let Some(idx @ (x, y)) = hull.pop_first() {
            if !visited.insert(idx) {
                continue;
            }
            for nbr in [
                (x < garden.width).then_some((x + 1, y)),
                x.checked_sub(1).map(|x| (x, y)),
                (y < garden.height).then_some((x, y + 1)),
                y.checked_sub(1).map(|y| (x, y)),
            ]
            .into_iter()
            .flatten()
            {
                if garden.get(nbr.0, nbr.1) != Some(Cell::Rock) {
                    next_hull.insert(nbr);
                }
            }
        }
        std::mem::swap(&mut next_hull, &mut hull);
    }
    visited.extend(hull);
    let (even, odd): (Vec<_>, Vec<_>) = visited.into_iter().partition(parity);
    let part1 = match parity(&start) ^ (loops % 2 == 0) {
        true => odd.len(),
        false => even.len(),
    };
    println!("{}, {}", garden.width, garden.height);
    println!("21.1: {}", part1);
}
