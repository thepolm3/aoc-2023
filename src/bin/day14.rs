use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Rock,
    // rock which can move
    Boulder,
    Ground,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
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

    fn transpose(&self) -> Self {
        let mut cells = vec![Cell::Ground; self.cells.len()];
        transpose::transpose(&self.cells, &mut cells, self.width, self.height);
        Self {
            cells,
            width: self.height,
            height: self.width,
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
        //SAFETY: x and y are both within bounds, so .add() should also always be in bounds
        //we know that self lives for 'a, and therefore so does the reference to Cell.
        //if self.height is 0 then this will not yield any pointers, and if self.height is nonzero
        //then all of our indexes are disjoint, provided x < self.width, which is asserted
        (0..self.height)
            .map(move |y| unsafe { &mut *self.cells.as_mut_ptr().add(self.width * y + x) })
    }

    fn tilt_north(&mut self) {
        for i in 0..self.width {
            let mut column = self.column_mut(i).collect_vec();
            let sections = column.split_mut(|x| **x == Cell::Rock).collect_vec();
            for section in sections {
                let n_boulders = section.iter().filter(|x| ***x == Cell::Boulder).count();
                for section in section.iter_mut().take(n_boulders) {
                    **section = Cell::Boulder;
                }
                for section in section.iter_mut().skip(n_boulders) {
                    **section = Cell::Ground;
                }
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day14.txt")?;
    //     let input = "O....#....
    // O.OO#....#
    // .....##...
    // OO.#O....O
    // .O.....O#.
    // O.#..O.#.#
    // ..O..#O..O
    // .......O..
    // #....###..
    // #OO..#....";
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
    let mut grid = Grid::new(cells, width);

    println!("{:?}", grid.column_mut(0).collect_vec());

    grid.tilt_north();

    println!("{:?}", grid.column_mut(0).collect_vec());

    let mut part1 = 0;
    for (i, row) in (0..grid.height)
        .rev()
        .map(|i| (i + 1, grid.row(grid.height - i - 1)))
    {
        part1 += row.filter(|&cell| *cell == Cell::Boulder).count() * i
    }

    println!("14.1: {}", part1);
    Ok(())
}
