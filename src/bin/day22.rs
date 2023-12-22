use std::fs::File;

use itertools::Itertools;
use ndarray::Array2;
use sprs::CsMat;
use std::io::Write;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum Axis {
    X,
    Y,
    Z,
}
impl Axis {
    fn rotate_axis(self) -> Self {
        use Axis::*;
        match self {
            X => Y,
            Y => Z,
            Z => X,
        }
    }
}

fn within(x: usize, lower: usize, upper: usize) -> bool {
    x >= lower && x < upper
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Block {
    direction: Axis,
    length: usize,
    base: (usize, usize, usize),
}
impl Block {
    fn new(lower: (usize, usize, usize), upper: (usize, usize, usize)) -> Option<Self> {
        if lower > upper {
            return Self::new(upper, lower);
        }
        Some(
            match (
                upper.0.abs_diff(lower.0),
                upper.1.abs_diff(lower.1),
                upper.2.abs_diff(lower.2),
            ) {
                (0, 0, d) => Block {
                    base: lower,
                    length: d + 1,
                    direction: Axis::Z,
                },
                (0, d, 0) => Block {
                    base: lower,
                    length: d + 1,
                    direction: Axis::Y,
                },
                (d, 0, 0) => Block {
                    base: lower,
                    length: d + 1,
                    direction: Axis::X,
                },
                _ => return None,
            },
        )
    }

    fn upper(&self) -> (usize, usize, usize) {
        use Axis::*;
        let (x, y, z) = self.base;
        match self.direction {
            X => (x + self.length - 1, y, z),
            Y => (x, y + self.length - 1, z),
            Z => (x, y, z + self.length - 1),
        }
    }

    fn intersect_xy(&self, other: &Block) -> bool {
        use Axis::*;
        match (self.direction, other.direction) {
            (X, Y) => {
                within(other.base.0, self.base.0, self.base.0 + self.length)
                    && within(self.base.1, other.base.1, other.base.1 + other.length)
            }
            (X, Z) => {
                within(other.base.0, self.base.0, self.base.0 + self.length)
                    && (self.base.1 == other.base.1)
            }
            (Y, Z) => {
                within(other.base.1, self.base.1, self.base.1 + self.length)
                    && (self.base.0 == other.base.0)
            }
            (X, X) => {
                (within(self.base.0, other.base.0, other.base.0 + other.length)
                    || within(other.base.0, self.base.0, self.base.0 + self.length))
                    && (self.base.1 == other.base.1)
            }
            (Y, Y) => {
                (within(self.base.1, other.base.1, other.base.1 + other.length)
                    || within(other.base.1, self.base.1, self.base.1 + self.length))
                    && (self.base.0 == other.base.0)
            }
            (Z, Z) => (self.base.0 == other.base.0) && (self.base.1 == other.base.1),
            (Y, X) | (Z, X) | (Z, Y) => other.intersect_xy(self),
        }
    }

    fn fall_to(self, z: usize) -> Self {
        Self {
            base: (self.base.0, self.base.1, z),
            direction: self.direction,
            length: self.length,
        }
    }

    fn verts_faces(&self) -> ([(f64, f64, f64); 8], [[isize; 4]; 6]) {
        let (x1, y1, z1) = self.base;
        let (x1, y1, z1) = (x1 as f64 + 0.1, y1 as f64 + 0.1, z1 as f64 + 0.1);
        let (x2, y2, z2) = self.upper();
        let (x2, y2, z2) = (x2 as f64 + 0.9, y2 as f64 + 0.9, z2 as f64 + 0.9);
        (
            [
                (x2, y1, z1),
                (x2, y2, z1),
                (x2, y2, z2),
                (x2, y1, z2),
                (x1, y2, z1),
                (x1, y1, z1),
                (x1, y1, z2),
                (x1, y2, z2),
            ],
            [
                [0, 1, 2, 3],
                [1, 4, 7, 2],
                [3, 2, 7, 6],
                [6, 7, 4, 5],
                [5, 0, 3, 6],
                [5, 4, 1, 0],
            ],
        )
    }

    // fn rotate_axis(self) -> Block {}
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let k = |x: &Block| (x.base.2, x.base, x.length, x.direction);
        k(self).cmp(&k(other))
    }
}

fn settle(blocks: Vec<Block>) -> (Vec<Block>, usize) {
    let mut settled = Vec::new();
    let mut moved = 0;
    for block in blocks {
        let z = 1 + settled
            .iter()
            .rev()
            .filter(|other| block.intersect_xy(other))
            .map(|other| other.upper().2)
            .max()
            .unwrap_or_default();
        if z != block.base.2 {
            moved += 1;
        }
        settled.push(block.fall_to(z));
    }
    (settled, moved)
}

fn export(blocks: &[Block], fname: &str, highlight: &[usize]) -> std::io::Result<()> {
    let mut buffer = File::create(fname)?;
    writeln!(buffer, "mtllib material.mtl")?;
    for (i, (v, f)) in blocks.iter().map(Block::verts_faces).enumerate() {
        for (x, y, z) in v {
            writeln!(buffer, "v {} {} {}", y, z, x)?;
        }

        let l = v.len() as isize;

        if highlight.contains(&i) {
            writeln!(buffer, "usemtl Red")?;
        } else {
            writeln!(buffer, "usemtl Blue")?;
        }
        for [x, y, z, w] in f {
            writeln!(buffer, "f {} {} {} {}", x - l, y - l, z - l, w - l)?;
        }
        for [x, y, z, w] in f {
            writeln!(buffer, "f {} {} {} {}", x - l, y - l, z - l, w - l)?;
        }
    }
    Ok(())
}

fn main() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,1,6~2,1,6
0,2,3~2,2,3
2,0,5~2,2,5
0,0,4~0,2,4
1,1,8~1,1,9";
    let input = std::fs::read_to_string("inputs/day22.txt").unwrap();
    // let input = "0,0,0~0,0,0";
    let mut blocks = input
        .lines()
        .map(|line| {
            let (lower, upper) = line.split_once('~').unwrap();
            let lower = lower
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let upper = upper
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            Block::new(lower, upper).unwrap()
        })
        .collect_vec();
    blocks.sort();
    export(&blocks, "start.obj", &Vec::new()).unwrap();

    //first we drop each block to its correct z position
    let fallen_blocks = settle(blocks).0;

    let mut load_bearing = Vec::new();
    for (i, block) in fallen_blocks.iter().enumerate() {
        load_bearing.push(
            fallen_blocks[..i]
                .iter()
                .enumerate()
                .filter(|(_, other)| {
                    (other.upper().2 + 1 == block.base.2) && (block.intersect_xy(other))
                }) // rests on each one on the layer below it intsersects with
                .map(|(j, _)| j)
                .collect_vec(),
        )
    }

    let disintegratable = (0..fallen_blocks.len())
        .filter(|i| {
            load_bearing
                .iter()
                .filter(|resting_on| resting_on.contains(i))
                .all(|resting_on| resting_on.len() > 1)
        })
        .collect_vec();

    println!("22.1: {}", disintegratable.len());

    export(&fallen_blocks, "fallen.obj", &disintegratable).unwrap();

    println!(
        "22.2: {}",
        (0..fallen_blocks.len())
            .map(|i| {
                let mut removed = fallen_blocks.clone();
                removed.remove(i);
                settle(removed).1
            })
            .sum::<usize>()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersection() {
        let b1 = Block::new((0, 0, 1), (0, 0, 5)).unwrap();
        let b2 = Block::new((0, 0, 5), (0, 0, 6)).unwrap();
        let b3 = Block::new((0, 0, 6), (0, 0, 10)).unwrap();
        let b4 = Block::new((0, 1, 1), (0, 1, 5)).unwrap();
        assert!(b1.intersect_xy(&b2));
        assert!(b1.intersect_xy(&b3));
        assert!(!b1.intersect_xy(&b4));

        let b1 = Block::new((0, 0, 50), (5, 0, 50)).unwrap();
        let b2 = Block::new((5, 0, 45), (6, 0, 45)).unwrap();
        let b3 = Block::new((6, 0, 12), (7, 0, 12)).unwrap();
        assert!(b1.intersect_xy(&b2));
        assert!(b2.intersect_xy(&b3));
        assert!(!b1.intersect_xy(&b3));

        let b1 = Block::new((0, 0, 50), (0, 5, 50)).unwrap();
        let b2 = Block::new((0, 5, 45), (0, 6, 45)).unwrap();
        let b3 = Block::new((0, 6, 12), (0, 7, 12)).unwrap();
        assert!(b1.intersect_xy(&b2));
        assert!(b2.intersect_xy(&b3));
        assert!(!b1.intersect_xy(&b3));

        let b1 = Block::new((0, 3, 50), (5, 3, 50)).unwrap();
        let b2 = Block::new((2, 0, 45), (2, 5, 45)).unwrap();
        assert!(b1.intersect_xy(&b2));
    }
}
