use itertools::Itertools;

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
                (0, 0, length) => Block {
                    base: lower,
                    length,
                    direction: Axis::Z,
                },
                (0, length, 0) => Block {
                    base: lower,
                    length,
                    direction: Axis::Y,
                },
                (length, 0, 0) => Block {
                    base: lower,
                    length,
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
            X => (x + self.length, y, z),
            Y => (x, y + self.length, z),
            Z => (x, y, z + self.length),
        }
    }

    fn intersect_xy(&self, other: &Block) -> bool {
        use Axis::*;
        match (self.direction, other.direction) {
            (X, Y) => {
                (self.base.0 <= other.base.0)
                    && (self.base.1 >= other.base.1)
                    && (self.base.0 + self.length >= other.base.0)
                    && (self.base.1 <= other.base.1 + other.length)
            }
            (X, Z) => {
                (self.base.0 <= other.base.0)
                    && (self.base.0 + self.length >= other.base.0)
                    && (self.base.1 == other.base.1)
            }
            (Y, Z) => {
                (self.base.1 <= other.base.1)
                    && (self.base.1 + self.length >= other.base.1)
                    && (self.base.1 == other.base.1)
            }
            (X, X) => {
                (((self.base.0 <= other.base.0) && (self.base.0 + self.length >= other.base.0))
                    || ((other.base.0 + other.length >= self.base.0)
                        && (other.base.0 <= self.base.0)))
                    && (self.base.1 == other.base.1)
            }
            (Y, Y) => {
                (((self.base.1 <= other.base.1) && (self.base.1 + self.length >= other.base.1))
                    || ((other.base.1 + other.length >= self.base.1)
                        && (other.base.1 <= self.base.1)))
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

    // fn rotate_axis(self) -> Block {}
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let k = |x: &Block| (x.base.2, x.base.1, x.base.0, x.length, x.direction);
        k(self).cmp(&k(other))
    }
}

fn main() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    let blocks = input
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
