use std::collections::VecDeque;

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
    let mut ranges: VecDeque<_> = seeds
        .chunks_exact(2)
        .map(|chunk| match chunk {
            [a, b] => [*a, *b],
            _ => panic!("Invalid seeds given"),
        })
        .collect();
    let mut mapped = VecDeque::new();

    for maps in map_sequence {
        while let Some([start, length]) = ranges.pop_front() {
            let end = start + length;
            for map in maps.clone() {
                let map_end = map.from + map.length;

                //range:   [  ]
                //map:   [      ]
                if map.from <= start && end <= map_end {
                    mapped.push_back([map.to + start - map.from, length]);
                    break;
                }

                //range:   [     ]     AND [            ]
                //map:        [      ]         [    ]
                if start < map.from && end > map.from {
                    ranges.push_back([start, map.from - start]);
                    ranges.push_back([map.from, end - map.from]);
                    break;
                }

                //range:   [         ]
                //map:   [      ]
                if start < map_end && end > map_end {
                    ranges.push_back([start, map_end - start]);
                    ranges.push_back([map_end, end - map_end]);
                    break;
                }

                //range:   [     ]
                //map:             [      ]
            }
        }
        std::mem::swap(&mut ranges, &mut mapped);
        mapped.clear();
    }
    println!(
        "5.2: {}",
        ranges.into_iter().min().context("empty values")?[0]
    );
    Ok(())
}
