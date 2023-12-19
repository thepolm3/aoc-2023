use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator::map,
    multi::{count, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Property {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuleList<'a> {
    list: Vec<(Constraint, &'a str)>,
    default: &'a str,
}

impl<'a> RuleList<'a> {
    fn map_part(&self, part: &Part) -> &str {
        for (c, m) in &self.list {
            if part.satisfies_constraint(c) {
                return m;
            }
        }
        self.default
    }

    fn map_range(&'a self, mut min: Part, mut max: Part) -> Vec<(&'a str, Part, Part)> {
        let mut result: Vec<(&str, Part, Part)> = Vec::new();
        for (Constraint(p, ordering, value), map_to) in &self.list {
            let value = *value;
            match ordering {
                //keep everything less than value
                Ordering::Less => {
                    if min.get_property(p) >= value {
                        //we can't satisfy the ineqality, so we continue as is
                        continue;
                    } else {
                        //we can satisfy the equality on this constraint
                        result.push((map_to, min, max.set(p, value - 1)));

                        //modify for next pass
                        min = min.set(p, value);
                    }
                }
                //keep everything strictly greater than value
                Ordering::Greater => {
                    if max.get_property(p) <= value {
                        //we can't satisfy the ineqality, so we continue as is
                        continue;
                    } else {
                        //we can satisfy the equality on this constraint
                        result.push((map_to, min.set(p, value + 1), max));

                        //modify for next pass
                        max = max.set(p, value);
                    }
                }
                Ordering::Equal => unreachable!(),
            }
        }
        result.push((self.default, min, max));
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn get_property(&self, p: &Property) -> u64 {
        use Property::*;
        match p {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.s,
        }
    }

    fn set(mut self, p: &Property, v: u64) -> Self {
        use Property::*;
        *match p {
            X => &mut self.x,
            M => &mut self.m,
            A => &mut self.a,
            S => &mut self.s,
        } = v;
        self
    }

    fn satisfies_constraint(&self, c: &Constraint) -> bool {
        self.get_property(&c.0).cmp(&c.2) == c.1
    }

    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn property(input: &str) -> IResult<&str, Property> {
    use Property::*;
    map(one_of("xmas"), |c| match c {
        'x' => X,
        'm' => M,
        'a' => A,
        's' => S,
        _ => unreachable!(),
    })(input)
}

fn relation(input: &str) -> IResult<&str, Ordering> {
    map(one_of("<=>"), |c| match c {
        '>' => Ordering::Greater,
        '=' => Ordering::Equal,
        '<' => Ordering::Less,
        _ => unreachable!(),
    })(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Constraint(Property, Ordering, u64);

fn constraint(input: &str) -> IResult<&str, Constraint> {
    map(
        tuple((property, relation, nom::character::complete::u64)),
        |(a, b, c)| Constraint(a, b, c),
    )(input)
}

fn rule(input: &str) -> IResult<&str, (Constraint, &str)> {
    separated_pair(constraint, tag(":"), alpha1)(input)
}

fn rule_list(input: &str) -> IResult<&str, RuleList> {
    map(
        delimited(
            tag("{"),
            pair(
                separated_list0(tag(","), rule),
                preceded(alt((tag(","), tag(""))), alpha1),
            ),
            tag("}"),
        ),
        |(list, default)| RuleList { list, default },
    )(input)
}

fn workflow(input: &str) -> IResult<&str, (&str, RuleList<'_>)> {
    pair(alpha1, rule_list)(input)
}

fn workflows(input: &str) -> IResult<&str, HashMap<&str, RuleList<'_>>> {
    map(separated_list0(line_ending, workflow), |rules| {
        rules.into_iter().collect()
    })(input)
}

fn part(input: &str) -> IResult<&str, Part> {
    use nom::character::complete::u64;
    map(
        delimited(
            tag("{"),
            permutation((
                preceded(tag("x="), u64),
                preceded(tag("m="), u64),
                preceded(tag("a="), u64),
                preceded(tag("s="), u64),
                tag(","),
                tag(","),
                tag(","),
            )),
            tag("}"),
        ),
        |values| Part {
            x: values.0,
            m: values.1,
            a: values.2,
            s: values.3,
        },
    )(input)
}

fn parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list0(line_ending, part)(input)
}

fn system(input: &str) -> IResult<&str, (HashMap<&str, RuleList<'_>>, Vec<Part>)> {
    separated_pair(workflows, count(line_ending, 2), parts)(input)
}

fn range_values(min: Part, max: Part) -> u64 {
    (max.x - min.x + 1) * (max.m - min.m + 1) * (max.a - min.a + 1) * (max.s - min.s + 1)
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day19.txt")?;

    let (_, (maps, inputs)) = system(&input).expect("invalid input");

    let mut accepted = 0;
    for part in inputs {
        let mut current = "in";
        while current != "R" {
            current = maps[current].map_part(&part);
            if current == "A" {
                accepted += part.sum();
                break;
            }
        }
    }
    println!("19.1: {:?}", accepted);

    let mut accepted = 0;
    let [min, max] = [
        Part {
            x: 1,
            m: 1,
            a: 1,
            s: 1,
        },
        Part {
            x: 4000,
            m: 4000,
            a: 4000,
            s: 4000,
        },
    ];

    let mut range_queue = VecDeque::from(vec![("in", min, max)]);
    while let Some((loc, min, max)) = range_queue.pop_front() {
        if loc == "A" {
            accepted += range_values(min, max);
            continue;
        }
        if loc == "R" {
            continue;
        }
        range_queue.extend(maps[loc].map_range(min, max));
    }
    println!("19.2: {:?}", accepted);

    Ok(())
}
