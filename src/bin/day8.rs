use once_cell::sync::Lazy;
use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

use anyhow::Context;
use itertools::Itertools;
use num::{traits::Euclid, BigInt, Integer};

static ZERO: Lazy<BigInt> = Lazy::new(|| BigInt::from(0));

//returns start and end point of first repeat in the instructions, with visits to final nodes
fn find_repeat_and_z_visits(
    starting: &str,
    instructions: &str,
    map: &HashMap<&str, (&str, &str)>,
) -> (usize, usize, Vec<usize>) {
    let cycle_length = instructions.len();
    let mut location = starting;

    let mut visited_at = HashMap::new();
    let mut final_visited = Vec::new();

    for (i, instruction) in instructions.chars().cycle().enumerate() {
        if location.ends_with('Z') {
            final_visited.push(i);
        }
        match visited_at.entry((i % cycle_length, location)) {
            Occupied(repeat_start) => return (*repeat_start.get(), i, final_visited),
            Vacant(e) => {
                e.insert(i);
            }
        }
        match instruction {
            'L' => location = map[location].0,
            'R' => location = map[location].1,
            _ => unreachable!(),
        };
    }
    unreachable!()
}

//gcd, u, v
fn bezoit(m: BigInt, n: BigInt) -> (BigInt, BigInt, BigInt) {
    if n > m {
        let (g, v, u) = bezoit(n, m);
        return (g, u, v);
    }
    let (q, r) = m.div_rem(&n);

    if r == *ZERO {
        return (n.clone(), BigInt::from(1), (q - 1) * n);
    }

    //found the GCD case
    if m.rem_euclid(&r) == *ZERO && n.rem_euclid(&r) == *ZERO {
        return (r, BigInt::from(1), -q);
    }
    // g = n*u + r*v
    // m = n*q + r
    // m * v - g = n*q*v - n*u
    let (g, u, v) = bezoit(n, r);
    (g, v.clone(), u - q * v)
}
fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day8.txt")?;

    let mut input = input.lines();
    let instructions = input.next().context("No lines")?;
    input.next();
    let map: HashMap<_, _> = input
        .map(|line| (&line[0..3], (&line[7..10], &line[12..15])))
        .collect();
    let mut current = "AAA";
    let part1 = instructions
        .chars()
        .cycle()
        .position(|instruction| {
            match instruction {
                'L' => current = map[current].0,
                'R' => current = map[current].1,
                _ => unreachable!(),
            };
            current == "ZZZ"
        })
        .unwrap()
        + 1;

    println!("8.1 {part1}");

    let mut current: Vec<_> = map
        .keys()
        .filter(|k: &&&str| k.ends_with('A'))
        .copied()
        .collect();
    current.sort();

    let repeats = current.into_iter().map(|x| {
        let (start, repeat, goals) = find_repeat_and_z_visits(x, instructions, &map);
        (
            BigInt::from(repeat - start),
            goals
                .into_iter()
                .map(|x| BigInt::from(x) % (repeat - start))
                .collect::<Vec<_>>(),
        )
    });
    let (n, solutions) = repeats
        .reduce(|(n, xs), (m, ys)| {
            let (g, u, v) = bezoit(n.clone(), m.clone());
            let lcm = n.clone() * m.clone() / g.clone();
            (
                lcm.clone(),
                xs.into_iter()
                    .cartesian_product(ys)
                    .map(|(a, b)| {
                        ((a * v.clone() * n.clone() + b * u.clone() * m.clone()) / g.clone())
                            .rem_euclid(&lcm)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap();

    let part2 = solutions
        .into_iter()
        .map(|x| if x == *ZERO { n.clone() } else { x })
        .min()
        .unwrap();

    println!("8.2 {part2}");

    Ok(())
}
