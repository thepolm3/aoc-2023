use anyhow::Context;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace0},
    combinator::map,
    multi::{count, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RangeMap {
    from: u64,
    to: u64,
    length: u64,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct MappedRegion((u64, u64));

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct UnmappedRegion((u64, u64));

#[derive(Debug, Clone, Eq, PartialEq)]
enum Intersection {
    None(UnmappedRegion),
    WholeRegion(MappedRegion),
    End(UnmappedRegion, MappedRegion),
    Middle([UnmappedRegion; 2], MappedRegion),
}

impl Intersection {
    fn get_mapped(&self) -> Option<(u64, u64)> {
        match self {
            Intersection::None(_) => None,
            Intersection::WholeRegion(MappedRegion(t)) => Some(*t),
            Intersection::End(_, MappedRegion(t)) => Some(*t),
            Intersection::Middle(_, MappedRegion(t)) => Some(*t),
        }
    }

    fn into_unmapped(self) -> impl Iterator<Item = (u64, u64)> {
        IntersectionUnmappedRegionIterator {
            inner: self,
            index: 0,
        }
        .map(|x| x.0)
    }
}

struct IntersectionUnmappedRegionIterator {
    inner: Intersection,
    index: usize,
}

impl Iterator for IntersectionUnmappedRegionIterator {
    type Item = UnmappedRegion;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match (self.index, &self.inner) {
            (1, Intersection::None(x)) => Some(*x),
            (1, Intersection::End(x, _)) => Some(*x),
            (1..=2, Intersection::Middle(x, _)) => Some(x[self.index - 1]),
            _ => None,
        }
    }
}

impl RangeMap {
    fn maps(&self, x: u64) -> Option<u64> {
        x.checked_sub(self.from)
            .and_then(|diff| (diff <= self.length).then_some(self.to + diff))
    }

    fn intersection(&self, (a, len): (u64, u64)) -> Intersection {
        if a + len <= self.from || a >= self.from + self.length {
            return Intersection::None(UnmappedRegion((a, len)));
        }

        if a >= self.from && a + len <= self.from + self.length {
            return Intersection::WholeRegion(MappedRegion((self.to + (a - self.from), len)));
        }

        if a <= self.from && a + len >= self.from + self.length {
            return Intersection::Middle(
                [
                    UnmappedRegion((a, self.from - a)),
                    UnmappedRegion((self.from + self.length, a + len - (self.from + self.length))),
                ],
                MappedRegion((self.to, self.length)),
            );
        }

        if a <= self.from {
            return Intersection::End(
                UnmappedRegion((a, self.from - a)),
                MappedRegion((self.to, len - (self.from - a))),
            );
        }

        Intersection::End(
            UnmappedRegion((self.from + self.length, a + len - (self.from + self.length))),
            MappedRegion((self.to + (a - self.from), self.from + self.length - a)),
        )
    }
}

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tag("seeds: "),
        separated_list1(tag(" "), nom::character::complete::u64),
    )(input)
}

fn title_line(input: &str) -> IResult<&str, (&str, &str)> {
    terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:"))(input)
}

fn range_map(input: &str) -> IResult<&str, RangeMap> {
    use nom::character::complete::u64;
    map(
        tuple((u64, multispace0, u64, multispace0, u64)),
        |(to, _, from, _, length)| RangeMap { from, to, length },
    )(input)
}

fn range_maps(input: &str) -> IResult<&str, Vec<RangeMap>> {
    preceded(
        pair(title_line, line_ending),
        separated_list1(line_ending, range_map),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<Vec<RangeMap>>)> {
    separated_pair(
        seeds,
        count(line_ending, 2),
        separated_list1(count(line_ending, 2), range_maps),
    )(input)
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day5.txt")?;
    let (_, (seeds, map_sequence)) = parse(&input).expect("invalid input");

    let part1 = seeds
        .iter()
        .map(|&seed| {
            map_sequence.iter().fold(seed, |old, range_maps| {
                range_maps
                    .iter()
                    .find_map(|range_map| range_map.maps(old))
                    .unwrap_or(old)
            })
        })
        .min()
        .context("empty list")?;

    let mut values = seeds
        .chunks_exact(2)
        .map(|chunk| match chunk {
            [a, b] => (*a, *b),
            _ => panic!("Invalid seeds given"),
        })
        .collect();

    for map_step in map_sequence {
        let mut mapped_values = Vec::new();
        for range_map in map_step {
            let mut new_values = Vec::new();
            for range in values {
                let intersection = range_map.intersection(range);

                mapped_values.extend(intersection.get_mapped());
                new_values.extend(intersection.into_unmapped());
            }
            values = new_values;
        }
        values.extend(mapped_values.into_iter());
    }

    println!("5.1: {part1:?}");
    println!(
        "5.2: {}",
        values.into_iter().min().context("empty values")?.0
    );

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn intersection() {
        let map: RangeMap = RangeMap {
            from: 5,
            to: 105,
            length: 5,
        };

        assert_eq!(
            map.intersection((3, 2)),
            Intersection::None(UnmappedRegion((3, 2)))
        );

        assert_eq!(
            map.intersection((3, 3)),
            Intersection::End(UnmappedRegion((3, 2)), MappedRegion((105, 1)))
        );

        assert_eq!(
            map.intersection((3, 4)),
            Intersection::End(UnmappedRegion((3, 2)), MappedRegion((105, 2)))
        );

        assert_eq!(
            map.intersection((8, 2)),
            Intersection::WholeRegion(MappedRegion((108, 2)))
        );

        assert_eq!(
            map.intersection((8, 15)),
            Intersection::End(UnmappedRegion((10, 13)), MappedRegion((108, 2)))
        );

        assert_eq!(
            map.intersection((5, 5)),
            Intersection::WholeRegion(MappedRegion((105, 5)))
        );
        assert_eq!(
            map.intersection((6, 3)),
            Intersection::WholeRegion(MappedRegion((106, 3)))
        );

        assert_eq!(
            map.intersection((1, 10)),
            Intersection::Middle(
                [UnmappedRegion((1, 4)), UnmappedRegion((10, 1))],
                MappedRegion((105, 5))
            ),
        );
    }
}
