#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Rock,
    Ash,
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
        let mut cells = vec![Cell::Ash; self.cells.len()];
        transpose::transpose(&self.cells, &mut cells, self.width, self.height);
        Self {
            cells,
            width: self.height,
            height: self.width,
        }
    }

    fn row(&self, i: usize) -> &[Cell] {
        &self.cells[self.width * i..(self.width * (i + 1))]
    }

    fn rows_eq(&self, r1: usize, r2: usize) -> bool {
        let row1 = &self.cells[self.width * r1..(self.width * (r1 + 1))];
        let row2 = &self.cells[self.width * r2..(self.width * (r2 + 1))];
        // println!("{r1}: {row1:?}");
        // println!("{r2}: {row2:?}");
        self.cells[self.width * r1..(self.width * (r1 + 1))]
            == self.cells[self.width * r2..(self.width * (r2 + 1))]
    }

    // fn row_diff(&self, r1: usize, r2: usize) -> usize {}

    fn row_mirror_line(&self) -> Option<usize> {
        let candidates: Vec<usize> = (0..(self.height - 1))
            .filter(|&i| (0..=i.min(self.height - i - 2)).all(|j| self.rows_eq(i - j, i + 1 + j)))
            .collect();
        candidates.first().copied()
    }
}

fn main() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let input = std::fs::read_to_string("inputs/day13.txt").unwrap();
    let mut grids: Vec<Grid> = Vec::new();
    let mut current = Vec::new();
    let mut width = 0;
    for line in input.lines() {
        if line.is_empty() {
            grids.push(Grid::new(current, width));
            current = Vec::new();
            continue;
        }
        width = line.len();
        for byte in line.as_bytes() {
            current.push(match byte {
                b'#' => Cell::Rock,
                b'.' => Cell::Ash,
                _ => panic!("Invalid input"),
            })
        }
    }
    if !current.is_empty() {
        grids.push(Grid::new(current, width));
    }

    let mut part1 = 0;
    for grid in grids {
        part1 += grid
            .transpose()
            .row_mirror_line()
            .map(|x| x + 1)
            .unwrap_or(0);
        part1 += grid.row_mirror_line().map(|x| x + 1).unwrap_or(0) * 100;
    }
    println!("{}", part1);
}
