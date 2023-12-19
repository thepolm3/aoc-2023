use std::{cmp::Ordering, collections::HashMap};

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator::{map, success},
    complete::take,
    multi::{count, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult, InputTake,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Property {
    X,
    M,
    A,
    S,
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

    fn satisfies_constraint(&self, c: &Constraint) -> bool {
        self.get_property(&c.0).cmp(&c.2) == c.1
    }

    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
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

fn main() -> anyhow::Result<()> {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    let input = std::fs::read_to_string("inputs/day19.txt")?;

    let (_, (maps, inputs)) = system(&input).expect("invalid input");

    let mut accepted = Vec::new();
    for part in inputs {
        let mut current = "in";
        while current != "R" {
            current = maps[current].map_part(&part);
            if current == "A" {
                accepted.push(part);
                break;
            }
        }
    }
    let part1 = accepted.iter().map(Part::sum).sum::<u64>();
    println!("{:?}", part1);

    Ok(())
}
