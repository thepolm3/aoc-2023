use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Rock,
    // rock which can move
    Boulder,
    Ground,
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
                        Cell::Rock => "#",
                        Cell::Boulder => "O",
                        Cell::Ground => " ",
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

    fn row(&self, y: usize) -> impl Iterator<Item = &Cell> {
        assert!(y < self.height);
        self.cells[self.width * y..(self.width * (y + 1))].iter()
    }

    fn row_mut(&mut self, y: usize) -> impl Iterator<Item = &mut Cell> {
        assert!(y < self.height);
        self.cells[self.width * y..(self.width * (y + 1))].iter_mut()
    }

    fn column(&self, x: usize) -> impl Iterator<Item = &Cell> {
        assert!(x < self.width);
        (0..self.height).map(move |y| &self.cells[self.width * y + x])
    }

    fn column_mut(&mut self, x: usize) -> impl Iterator<Item = &mut Cell> {
        assert!(x < self.width);
        assert!(self.width * (self.height - 1) + x < self.cells.len());

        //SAFETY: x and y are both within bounds, so .add() should also always be in bounds
        //we know that self lives for 'a, and therefore so does the reference to Cell.
        //if self.height is 0 then this will not yield any references, and if self.height is nonzero
        //then all of our indexes are disjoint, provided x < self.width, which is asserted.
        //we do an additonal assert to ensure that the internal cells buffer is of the correct length
        (0..self.height)
            .map(move |y| unsafe { &mut *self.cells.as_mut_ptr().add(self.width * y + x) })
    }

    //splits the given axis at `Rock`s, in each chunk moving either `Boulder` or `Ground` to
    //the start, depending on which is `heavier`
    fn tilt<'a>(axis: impl Iterator<Item = &'a mut Cell>, heavier: Cell) {
        let lighter = match heavier {
            Cell::Rock => panic!(),
            Cell::Boulder => Cell::Ground,
            Cell::Ground => Cell::Boulder,
        };

        let mut column = axis.collect_vec();
        let sections = column.split_mut(|x| **x == Cell::Rock).collect_vec();
        for section in sections {
            let n_rising = section.iter().filter(|x| ***x == heavier).count();
            for section in section.iter_mut().take(n_rising) {
                **section = heavier;
            }
            for section in section.iter_mut().skip(n_rising) {
                **section = lighter;
            }
        }
    }

    fn tilt_north(&mut self) {
        for i in 0..self.width {
            Grid::tilt(self.column_mut(i), Cell::Boulder);
        }
    }

    fn tilt_south(&mut self) {
        for i in 0..self.width {
            Grid::tilt(self.column_mut(i), Cell::Ground);
        }
    }

    fn tilt_east(&mut self) {
        for i in 0..self.height {
            Grid::tilt(self.row_mut(i), Cell::Ground);
        }
    }

    fn tilt_west(&mut self) {
        for i in 0..self.height {
            Grid::tilt(self.row_mut(i), Cell::Boulder);
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn north_load(&self) -> usize {
        (0..self.height)
            .rev()
            .map(|i| (i + 1, self.row(self.height - i - 1)))
            .map(|(i, row)| row.filter(|&cell| *cell == Cell::Boulder).count() * i)
            .sum()
    }
}

fn cycle_repeats(mut grid: Grid) -> (usize, usize, Grid) {
    let mut grids = HashMap::new();

    for i in 0.. {
        let last_seen = grids.insert(grid.clone(), i);
        if let Some(j) = last_seen {
            return (j, i, grid);
        }
        grid.cycle();
    }
    unreachable!()
}

fn cycle_n(mut grid: Grid, n: usize) -> Grid {
    let (first, last, mut cycled_grid) = cycle_repeats(grid.clone());
    if n <= last {
        for _ in 0..n {
            grid.cycle();
        }
        return grid;
    }
    let n = (n - first) % (last - first);
    for _ in 0..n {
        cycled_grid.cycle();
    }
    cycled_grid
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day14.txt")?;
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|char| match char {
                'O' => Cell::Boulder,
                '#' => Cell::Rock,
                '.' => Cell::Ground,
                _ => panic!("Invalid input"),
            })
        })
        .collect();
    let grid = Grid::new(cells, width);
    let mut p1grid = grid.clone();
    p1grid.tilt_north();

    println!("14.1: {:?}", p1grid.north_load());
    println!("14.2: {:?}", cycle_n(grid, 1000000000).north_load());

    // println!("14.1: {}", part1);
    Ok(())
}
