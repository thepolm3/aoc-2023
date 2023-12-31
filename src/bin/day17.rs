use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fmt::Debug,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl Direction {
    fn move_from(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Self::North => y.checked_sub(1).map(|y| (x, y)),
            Self::East => x.checked_add(1).map(|x| (x, y)),
            Self::South => y.checked_add(1).map(|y| (x, y)),
            Self::West => x.checked_sub(1).map(|x| (x, y)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct ExplorePosition {
    cost: u32,
    moved: usize,
    d: Direction,
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Grid {
    cells: Vec<u32>,
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
                write!(f, "{cell}")?
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
    fn new(cells: Vec<u32>, width: usize) -> Self {
        let height = cells.len() / width;
        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u32> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get(y * self.width + x).copied()
    }
}

fn part1(grid: &Grid) -> Option<u32> {
    use Direction::*;
    let mut to_explore = BTreeSet::new();
    let mut visited = HashMap::new();
    to_explore.insert(ExplorePosition {
        x: 0,
        y: 0,
        d: East,
        moved: 0,
        cost: 0,
    });
    to_explore.insert(ExplorePosition {
        x: 0,
        y: 0,
        d: South,
        moved: 0,
        cost: 0,
    });
    while let Some(ExplorePosition {
        x,
        y,
        d,
        moved,
        cost,
    }) = to_explore.pop_first()
    {
        if let Some(m) = visited.get_mut(&(x, y, d)) {
            if *m <= moved {
                //we've been here before with more moves available
                continue;
            }
            *m = moved;
        } else {
            visited.insert((x, y, d), moved);
        }
        if (x, y) == (grid.width - 1, grid.height - 1) {
            return Some(cost);
        }
        for explore_d in [North, East, South, West] {
            if moved == 3 && d == explore_d || explore_d == d.opposite() {
                continue;
            }
            if let Some((x2, y2)) = explore_d.move_from(x, y) {
                if let Some(position_cost) = grid.get(x2, y2) {
                    let e2 = ExplorePosition {
                        x: x2,
                        y: y2,
                        d: explore_d,
                        moved: if d == explore_d { moved + 1 } else { 1 },
                        cost: position_cost + cost,
                    };
                    to_explore.insert(e2);
                }
            }
        }
    }
    None
}

fn part2(grid: &Grid) -> Option<u32> {
    use Direction::*;
    let mut to_explore = BTreeSet::new();
    let mut visited = HashMap::new();
    to_explore.insert(ExplorePosition {
        x: 0,
        y: 0,
        d: East,
        moved: 0,
        cost: 0,
    });
    to_explore.insert(ExplorePosition {
        x: 0,
        y: 0,
        d: South,
        moved: 0,
        cost: 0,
    });
    while let Some(ExplorePosition {
        x,
        y,
        d,
        moved,
        cost,
    }) = to_explore.pop_first()
    {
        if moved >= 4 {
            if let Some(m) = visited.get_mut(&(x, y, d)) {
                if *m <= moved {
                    //we've been here before with more moves available
                    continue;
                }
                *m = moved;
            } else {
                visited.insert((x, y, d), moved);
            }
        }
        if (x, y) == (grid.width - 1, grid.height - 1) && moved >= 4 {
            return Some(cost);
        }
        for explore_d in [North, East, South, West] {
            if moved < 4 && d != explore_d
                || moved == 10 && d == explore_d
                || explore_d == d.opposite()
            {
                continue;
            }
            if let Some((x2, y2)) = explore_d.move_from(x, y) {
                if let Some(position_cost) = grid.get(x2, y2) {
                    to_explore.insert(ExplorePosition {
                        x: x2,
                        y: y2,
                        d: explore_d,
                        moved: if d == explore_d { moved + 1 } else { 1 },
                        cost: position_cost + cost,
                    });
                }
            }
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day17.txt")?;
    let width = input.lines().next().unwrap().len();
    let grid = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_string().parse::<u32>().unwrap()))
        .collect();
    let grid = Grid::new(grid, width);

    println!("17.1: {:?}", part1(&grid));
    println!("17.2: {:?}", part2(&grid));

    Ok(())
}
