use anyhow::Context;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, newline},
    combinator::{map, map_res},
    multi::{count, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn digit1(input: &str) -> IResult<&str, u32> {
    map_res(nom::character::complete::digit1, str::parse)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RangeMap {
    from: u32,
    to: u32,
    length: u32,
}

impl RangeMap {
    fn map(&self, x: u32) -> u32 {
        if x >= self.from && x - self.from <= self.length {
            return self.to + (x - self.from);
        }
        x
    }
}

fn seeds(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(tag("seeds: "), separated_list1(tag(" "), digit1))(input)
}

fn title_line(input: &str) -> IResult<&str, (&str, &str)> {
    terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:"))(input)
}

fn range_map(input: &str) -> IResult<&str, RangeMap> {
    map(
        tuple((digit1, multispace0, digit1, multispace0, digit1)),
        |(from, _, to, _, length)| RangeMap { from, to, length },
    )(input)
}

fn range_maps(input: &str) -> IResult<&str, Vec<RangeMap>> {
    preceded(
        pair(title_line, newline),
        separated_list1(newline, range_map),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<Vec<RangeMap>>)> {
    let (input, seeds) = seeds(input)?;
    let (input, _) = count(newline, 2)(input)?;
    let (input, maps) = separated_list1(count(newline, 2), range_maps)(input)?;
    Ok((input, (seeds, maps)))
}

fn main() -> anyhow::Result<()> {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    //fails with unclear lifetime error without leaking -- apparently parsing `input` requires that it be 'static
    let input: &'static str = std::fs::read_to_string("inputs/day5.txt")?.leak();
    let (_, (seeds, map_sequence)) = parse(input)?;

    let part1 = seeds
        .iter()
        .map(|&seed| {
            map_sequence.iter().fold(seed, |old, range_maps| {
                range_maps.iter().fold(old, |x, range_map| range_map.map(x))
            })
        })
        .min()
        .context("empty list")?;

    println!("{part1:?}");

    Ok(())
}
