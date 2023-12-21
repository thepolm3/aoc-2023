use std::collections::{BTreeMap, BTreeSet, HashSet};

use itertools::Itertools;
use num::Integer;

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

    fn wrap_coords(&self, (x, y): (usize, usize)) -> (usize, usize) {
        (x % self.width, y % self.height)
    }
}

fn parity((x, y): &(usize, usize)) -> bool {
    ((x + y) % 2) == 0
}

// N E S W
fn n_visited_after(garden: &Garden, start: (usize, usize), steps: usize) -> usize {
    let mut visited = BTreeSet::new();
    let mut hull = BTreeSet::new();
    let mut next_hull = BTreeSet::new();
    hull.insert(start);

    for _ in 0..steps {
        while let Some(idx @ (x, y)) = hull.pop_first() {
            if !visited.insert(idx) {
                continue;
            }
            for nbr in [
                (x + 1 < garden.width).then_some((x + 1, y)),
                x.checked_sub(1).map(|x| (x, y)),
                (y + 1 < garden.height).then_some((x, y + 1)),
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
    match parity(&start) ^ (steps % 2 == 0) {
        true => odd.len(),
        false => even.len(),
    }
}

//correct on sample data
fn n_visited_after_looping(garden: &Garden, (x, y): (usize, usize), steps: usize) -> usize {
    let start: (isize, isize) = (x as isize, y as isize);
    let mut visited = BTreeSet::new();
    let mut hull = BTreeSet::new();
    let mut next_hull = BTreeSet::new();
    hull.insert(start);

    for _ in 0..steps {
        while let Some(idx @ (x, y)) = hull.pop_first() {
            if !visited.insert(idx) {
                continue;
            }
            for nbr in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                if garden.get(
                    nbr.0.rem_euclid(garden.width as isize) as usize,
                    nbr.1.rem_euclid(garden.height as isize) as usize,
                ) != Some(Cell::Rock)
                {
                    next_hull.insert(nbr);
                }
            }
        }
        std::mem::swap(&mut next_hull, &mut hull);
    }
    visited.extend(hull);
    let (even, odd): (Vec<_>, Vec<_>) = visited.into_iter().partition(|(x, y)| ((x + y) % 2) == 0);
    match (((start.0 + start.1) % 2) == 0) ^ (steps % 2 == 0) {
        true => odd.len(),
        false => even.len(),
    }
}

fn n_visited_looping_fast(steps: usize, start: (usize, usize), garden: &Garden) -> usize {
    //first we note that for our input our entire starting row/column is clear
    assert!((0..garden.height).all(|y| garden.get(start.0, y) == Some(Cell::Plot)));
    assert!((0..garden.width).all(|x| garden.get(x, start.1) == Some(Cell::Plot)));

    //we assume square from now on
    assert!(garden.width == garden.height);
    assert!(garden.width % 2 == 1);

    //and that our starting square is in the middle
    assert!(start.0 * 2 + 1 == garden.width);
    assert!(start.1 * 2 + 1 == garden.height);

    //we also assume that the two corners can be reached in minimum distance,
    //this is a stronger property (the gutters of the data) but it does hold for the input data
    assert!((0..garden.height).all(|y| garden.get(0, y) == Some(Cell::Plot)));
    assert!((0..garden.height).all(|x| garden.get(x, garden.height - 1) == Some(Cell::Plot)));
    assert!((0..garden.height).all(|x| garden.get(x, 0) == Some(Cell::Plot)));
    assert!((0..garden.height).all(|y| garden.get(garden.width - 1, y) == Some(Cell::Plot)));

    //we also assume that the corners are the furthest two points can be from
    //each other, but we could do the algorithm without it

    //due to the gutters at the edge, this means every single garden will be entered
    //from the corner closest to the center, except the ones directly orthogonal to our start

    // A map of our universe looks like this, with each symbol a garden
    //       ^
    //      /c\
    //     /e|e\
    //    /ex|xe\
    //   <c--S--c>
    //    \ex|xe/
    //     \e|e/
    //      \c/
    //       v

    // first we move to any corner, it takes us at least start * 2, and due to
    //our assumptions, we have a clear path to each corner. It takes us 2 more steps
    //to get to the opposite corner of the next grid
    let starting_overflow = steps.checked_sub(start.0 + start.1 + 2);

    //now we're in the corner with `starting_overflow` steps left to take.
    //to get to another matching corner takes us one width worth of steps
    let to_next_garden = garden.width;

    //so the outermost garden will be `radius` gardens away, with `remaining` steps remaining
    let (radius, remaining_outer_edge) = starting_overflow
        .unwrap_or_default()
        .div_rem(&to_next_garden);

    //the number of very edge gardens, less that half full (\).
    let n_outer_edge = if starting_overflow.is_none() {
        0
    } else {
        radius + 1
    };

    //the number of edge gardens which are more than half full (e)
    let n_inner_edge = radius;

    //for these we have more steps remaining from their corners. This should be less than
    // or equal to `to_next_corner`
    let remaining_inner_edge = remaining_outer_edge + garden.width;

    // the rest in the diagram (the xs) are completely full. This will be a triangular number
    let interior_radius = radius.saturating_sub(1);
    let full_interior = (interior_radius) * (interior_radius + 1) / 2;

    // The full interior which have the same parity as the center square
    //since we're in the case the grid is odd, every other garden will have opposite parity
    // We'll have odd / even parity in the interior as follows (assuming c is even)
    //   |eo\
    //   |oeo\
    //   |eoeo\
    //   c----->
    // o's the other parity will be the even triangular number with half its size
    let full_interior_other_parity = (interior_radius / 2) * (interior_radius / 2 + 1);
    let full_interior_center_parity = full_interior - full_interior_other_parity;

    //now we consider the gardens in line with our starting square. We will begin in the center
    //of each new tile we explore
    let corner_overflow = steps.checked_sub(start.0 + 1);

    let (corner_radius, corner_remaining_outer) =
        corner_overflow.unwrap_or_default().div_rem(&to_next_garden);

    // there is one outer corner

    let n_corner_outer = (corner_overflow.is_some()) as usize;
    let n_corner_inner = (corner_radius > 0) as usize;
    // there is one inner corner (c) which has an extra garden width to go. This may be full
    // but there's no problem with special casing it
    let corner_remaining_inner = corner_remaining_outer + garden.width;

    //all others in line with the edge are full
    let full_inline = corner_radius.saturating_sub(1);

    let full_inline_center_parity = full_inline / 2;
    let full_inline_other_parity = full_inline - full_inline_center_parity;

    let full_center = 1;

    let mut sum = 0;

    let full_squares_center_parity =
        (full_inline_center_parity * 4 + full_interior_center_parity * 4 + full_center)
            * n_visited_after(garden, start, steps);
    let full_squares_other_parity = (full_inline_other_parity * 4 + full_interior_other_parity * 4)
        * n_visited_after(garden, start, steps + 1);

    // dbg!(
    //     n_corner_outer,
    //     corner_remaining_outer,
    //     n_corner_inner,
    //     corner_remaining_inner,
    //     n_outer_edge,
    //     remaining_outer_edge,
    //     n_inner_edge,
    //     remaining_inner_edge,
    //     full_inline,
    //     full_inline_center_parity,
    //     full_interior,
    //     full_interior_center_parity,
    //     full_center,
    //     full_squares_center_parity,
    //     full_squares_other_parity
    // );

    sum += full_squares_center_parity + full_squares_other_parity;
    for corner in [
        (0, 0),
        (garden.width - 1, 0),
        (0, garden.height - 1),
        (garden.width - 1, garden.height - 1),
    ] {
        let outer_edge_squares =
            n_outer_edge * n_visited_after(garden, corner, remaining_outer_edge);
        let inner_edge_squares =
            n_inner_edge * n_visited_after(garden, corner, remaining_inner_edge);

        sum += outer_edge_squares + inner_edge_squares;
    }

    for edge in [
        (start.0, 0),
        (start.0, garden.width - 1),
        (0, start.1),
        (garden.width - 1, start.1),
    ] {
        let outer_corner_squres =
            n_corner_outer * n_visited_after(garden, edge, corner_remaining_outer);
        let inner_corner_squares =
            n_corner_inner * n_visited_after(garden, edge, corner_remaining_inner);

        sum += outer_corner_squres + inner_corner_squares;
    }
    sum
}

fn parse(input: &str) -> Option<((usize, usize), Garden)> {
    let start = if let (start_y, Some(start_x)) = input
        .lines()
        .map(|line| line.chars().position(|c| c == 'S'))
        .find_position(Option::is_some)?
    {
        (start_x, start_y)
    } else {
        panic!("No start")
    };

    Some((start, Garden::from_str(input)?))
}

fn main() {
    let input = std::fs::read_to_string("inputs/day21.txt").unwrap();

    let (start, garden) = parse(&input).unwrap();
    println!("21.2: {}", n_visited_looping_fast(64, start, &garden));

    let steps = 26501365;
    println!("21.2: {}", n_visited_looping_fast(steps, start, &garden));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        let (start, garden) = parse(input).unwrap();

        assert_eq!(n_visited_after(&garden, start, 0), 1);
        assert_eq!(n_visited_after(&garden, start, 1), 2);
        assert_eq!(n_visited_after(&garden, start, 2), 4);
        assert_eq!(n_visited_after(&garden, start, 3), 6);
        assert_eq!(n_visited_after(&garden, start, 6), 16);
    }

    #[test]
    fn test_looping_naiive() {
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
        let (start, garden) = parse(input).unwrap();

        assert_eq!(n_visited_after_looping(&garden, start, 6), 16);
        assert_eq!(n_visited_after_looping(&garden, start, 10), 50);
        assert_eq!(n_visited_after_looping(&garden, start, 50), 1594);
        assert_eq!(n_visited_after_looping(&garden, start, 100), 6536);
        assert_eq!(n_visited_after_looping(&garden, start, 500), 167004);
        // assert_eq!(n_visited_after_looping(&garden, start, 1000), 668697);
        // assert_eq!(n_visited_after_looping(&garden, start, 5000), 16733044);
    }

    #[test]
    fn test_looping_agrees() {
        let input = std::fs::read_to_string("inputs/day21.txt").unwrap();
        let (start, garden) = parse(&input).unwrap();
        for steps in (0..10).map(|x| x * 10 + 1) {
            assert_eq!(
                n_visited_looping_fast(steps, start, &garden),
                n_visited_after_looping(&garden, start, steps)
            );
        }
    }

    #[test]
    fn test_without_rocks() {
        let input = "...
.S.
...";
        let (start, garden) = parse(input).unwrap();

        //will always produce squares, due to no rocks
        assert_eq!(n_visited_looping_fast(8, start, &garden), 81);
        assert_eq!(n_visited_looping_fast(9, start, &garden), 100);
        assert_eq!(n_visited_looping_fast(10, start, &garden), 121);
        assert_eq!(n_visited_looping_fast(11, start, &garden), 144);
        assert_eq!(n_visited_looping_fast(12, start, &garden), 169);
        assert_eq!(n_visited_looping_fast(13, start, &garden), 196);
        assert_eq!(n_visited_looping_fast(101, start, &garden), 10404);
        assert_eq!(n_visited_looping_fast(1003, start, &garden), 1008016);
    }

    #[test]
    fn test_with_rock() {
        let input = ".....
...#.
..S..
.....
.....";
        let (start, garden) = parse(input).unwrap();

        //worked out by hand
        assert_eq!(n_visited_looping_fast(8, start, &garden), 79);
        assert_eq!(n_visited_looping_fast(9, start, &garden), 96);
        assert_eq!(n_visited_looping_fast(10, start, &garden), 115);
        assert_eq!(n_visited_looping_fast(11, start, &garden), 140);
        assert_eq!(n_visited_looping_fast(12, start, &garden), 160);
        assert_eq!(n_visited_looping_fast(13, start, &garden), 190);
    }
}
