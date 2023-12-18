use itertools::Itertools;

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_str(s: &str) -> Option<Self> {
        use Direction::*;
        Some(match s {
            "U" => North,
            "L" => East,
            "D" => South,
            "R" => West,
            _ => return None,
        })
    }
    fn from_byte(byte: u8) -> Option<Self> {
        use Direction::*;
        Some(match byte {
            b'0' => West,
            b'1' => South,
            b'2' => East,
            b'3' => North,
            _ => return None,
        })
    }
}
fn metres_dug(v: &[(Direction, i64)]) -> i64 {
    use Direction::*;
    let boundary_points = v.iter().map(|(_, x)| x).sum::<i64>();

    //shoelace formula
    let area = v
        .iter()
        .scan((0, 0), |(x, y), (d, n)| {
            match *d {
                North => *y -= n,
                South => *y += n,
                East => *x -= n,
                West => *x += n,
            };
            Some((*x, *y))
        })
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| (y1 + y2) * (x1 - x2))
        .sum::<i64>()
        / 2;

    area + (boundary_points / 2) + 1
}
fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day18.txt")?;
    let part1 = metres_dug(
        &input
            .lines()
            .filter_map(|line| {
                let line = line.split_whitespace().collect_vec();
                Some((Direction::from_str(line[0])?, line[1].parse::<i64>().ok()?))
            })
            .collect_vec(),
    );

    println!("18.1: {part1}");
    let part2 = metres_dug(
        &input
            .lines()
            .filter_map(|line| {
                let line = line.split_whitespace().collect_vec();
                let hexcode = line[2].strip_prefix("(#")?.strip_suffix(')')?;
                Some((
                    Direction::from_byte(hexcode.as_bytes()[5])?,
                    <i64>::from_str_radix(&hexcode[0..5], 16).ok()?,
                ))
            })
            .collect_vec(),
    );
    println!("18.1: {part2}");

    //using picks theorem, i + b = a + b.2 + 1

    Ok(())
}
