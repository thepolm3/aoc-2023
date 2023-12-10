use std::collections::HashSet;

use anyhow::Context;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NE,
    NS,
    NW,
    ES,
    EW,
    SW,
}

impl Pipe {
    fn from_char(x: char) -> Option<Self> {
        use Pipe::*;
        match x {
            'L' => Some(NE),
            '|' => Some(NS),
            'J' => Some(NW),
            'F' => Some(ES),
            '-' => Some(EW),
            '7' => Some(SW),
            _ => None,
        }
    }

    fn free_directions(&self) -> [Direction; 2] {
        use Direction::*;
        use Pipe::*;
        match self {
            NE => [North, East],
            NS => [North, South],
            NW => [North, West],
            ES => [East, South],
            EW => [East, West],
            SW => [South, West],
        }
    }

    fn has_access(&self, d: Direction) -> bool {
        self.free_directions().contains(&d)
    }

    fn next_direction(&self, d: Direction) -> Direction {
        let [a, b] = self.free_directions();
        match d.opposite() {
            x if x == a => b,
            x if x == b => a,
            _ => panic!("incorrect approach"),
        }
    }
}
struct Grid {
    inner: Vec<Option<Pipe>>,
    width: usize,
    height: usize,
    start: (usize, usize),
}

impl Grid {
    fn from_str(input: &str) -> Option<Self> {
        let lines: Vec<_> = input.lines().collect();
        let width = lines.first()?.len();
        let height = lines.len();
        let start = if let (start_y, Some(start_x)) = lines
            .iter()
            .map(|line| line.chars().position(|c| c == 'S'))
            .find_position(Option::is_some)?
        {
            (start_x, start_y)
        } else {
            unreachable!()
        };

        let inner = lines
            .into_iter()
            .flat_map(|line| line.chars().map(Pipe::from_char))
            .collect();

        Some(Self {
            inner,
            width,
            height,
            start,
        })
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<Pipe> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.inner[self.width * y + x]
    }
}

struct Traveller<'a> {
    position: (usize, usize),
    heading: Direction,
    grid: &'a Grid,
}

impl<'a> Traveller<'a> {
    fn new(grid: &'a Grid) -> Self {
        use Direction::*;
        let position @ (x, y) = grid.start;

        for (x, y, heading) in [
            (x, y.saturating_sub(1), North),
            (x, y + 1, South),
            (x + 1, y, East),
            (x.saturating_sub(1), y, West),
        ] {
            if let Some(pipe) = grid.get((x, y)) {
                if pipe.has_access(heading.opposite()) {
                    // println!("{:?} {:?}", position, heading);
                    return Self {
                        position,
                        heading,
                        grid,
                    };
                }
            }
        }

        panic!("Invalid grid")
    }
}

impl<'a> Iterator for Traveller<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.position;
        use Direction::*;
        self.position = match self.heading {
            North => (x, y - 1),
            East => (x + 1, y),
            South => (x, y + 1),
            West => (x - 1, y),
        };

        self.heading = self.grid.get(self.position)?.next_direction(self.heading);
        // println!("{:?} {:?}", self.position, self.heading);

        Some(self.position)
    }
}

fn main() -> anyhow::Result<()> {
    use Direction::*;
    let input = std::fs::read_to_string("inputs/day10.txt")?;
    let grid = Grid::from_str(&input).context("Invalid grid")?;

    let traveller = Traveller::new(&grid);
    let mut l = HashSet::new();

    l.insert(traveller.position);
    l.extend(traveller);

    println!("10.1: {}", (l.len()) / 2);

    let mut inside = 0;
    let mut inside_line = false;
    let mut last_corner = None;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if l.contains(&(x, y)) {
                if let Some(p) = grid.get((x, y)) {
                    if p == Pipe::NS
                        || (p == Pipe::NW && last_corner == Some(South))
                        || (p == Pipe::SW && last_corner == Some(North))
                    {
                        inside_line = !inside_line;
                    }

                    match p {
                        Pipe::NE => last_corner = Some(North),
                        Pipe::NW => last_corner = None,
                        Pipe::ES => last_corner = Some(South),
                        Pipe::SW => last_corner = None,
                        _ => {}
                    }
                }
            } else if inside_line {
                inside += 1;
            }
        }
    }

    println!("10.2: {}", inside);

    Ok(())
}
