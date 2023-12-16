use core::panic;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Empty,
    NorthEastMirror,
    SouthEastMirror,
    VerticalSplitter,
    HorizontalSplitter,
    Light,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "-")?;
        }
        writeln!(f, "+")?;
        for line in self.cells.iter().chunks(self.width).into_iter() {
            write!(f, "|")?;
            for cell in line {
                write!(
                    f,
                    "{}",
                    match cell {
                        Cell::Empty => ' ',
                        Cell::NorthEastMirror => '/',
                        Cell::SouthEastMirror => '\\',
                        Cell::VerticalSplitter => '|',
                        Cell::HorizontalSplitter => '-',
                        Cell::Light => '#',
                    }
                )?
            }
            writeln!(f, "|")?;
        }
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "-")?;
        }
        writeln!(f, "+")
    }
}

impl Grid {
    fn new(cells: Vec<Cell>, width: usize) -> Self {
        let height = cells.len() / width;
        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get(y * self.width + x).copied()
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get_mut(y * self.width + x)
    }
}

fn main() -> anyhow::Result<()> {
    use Cell::*;
    use Direction::*;
    let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    let input = std::fs::read_to_string("inputs/day16.txt")?;
    let width = input.lines().next().unwrap().len();
    let grid = input
        .lines()
        .flat_map(|line| {
            line.as_bytes().iter().map(|x| match x {
                b'|' => VerticalSplitter,
                b'-' => HorizontalSplitter,
                b'/' => NorthEastMirror,
                b'\\' => SouthEastMirror,
                _ => Empty,
            })
        })
        .collect();
    let grid = Grid::new(grid, width);
    let mut visualisation = grid.clone();
    // println!("{:?}", grid);

    let mut visited = HashSet::new();
    let mut beam_positions = VecDeque::from(vec![(0, 0, East)]);
    while let Some((x, y, d)) = beam_positions.pop_front() {
        let cell = grid.get(x, y);
        if cell.is_none() {
            //went off the edge
            continue;
        }
        if !visited.insert((x, y, d)) {
            continue;
        };
        let v = visualisation.get_mut(x, y).unwrap();
        if *v == Empty {
            *v = Light;
        }
        let cell = cell.unwrap();
        match (d, cell) {
            (North, Empty)
            | (North, VerticalSplitter)
            | (East, NorthEastMirror)
            | (West, SouthEastMirror) => {
                if let Some(y) = y.checked_sub(1) {
                    beam_positions.push_back((x, y, North));
                }
            }
            (North, NorthEastMirror)
            | (East, Empty)
            | (East, HorizontalSplitter)
            | (South, SouthEastMirror) => beam_positions.push_back((x + 1, y, East)),
            (East, SouthEastMirror)
            | (South, Empty)
            | (South, VerticalSplitter)
            | (West, NorthEastMirror) => beam_positions.push_back((x, y + 1, South)),
            (North, SouthEastMirror)
            | (South, NorthEastMirror)
            | (West, Empty)
            | (West, HorizontalSplitter) => {
                if let Some(x) = x.checked_sub(1) {
                    beam_positions.push_back((x, y, West));
                }
            }
            (North, HorizontalSplitter) | (South, HorizontalSplitter) => {
                beam_positions.extend([(x, y, West), (x, y, East)])
            }
            (East, VerticalSplitter) | (West, VerticalSplitter) => {
                beam_positions.extend([(x, y, North), (x, y, South)])
            }
            _ => panic!(),
        }
    }
    println!("{:?}", visualisation);
    let part1 = visited
        .into_iter()
        .map(|(a, b, _)| (a, b))
        .inspect(|&(x, y)| assert!(x < grid.width && y < grid.height))
        .unique()
        .count();
    println!("16.1: {part1}");

    Ok(())
}
