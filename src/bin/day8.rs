use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap, HashSet,
    },
    io::repeat,
};

use anyhow::Context;
use num::{BigInt, Integer};

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
fn main() -> anyhow::Result<()> {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let input = std::fs::read_to_string("inputs/day8.txt")?;
    let mut input = input.lines();
    let instructions = input.next().context("No lines")?;
    input.next();
    let map: HashMap<_, _> = input
        .map(|line| (&line[0..3], (&line[7..10], &line[12..15])))
        .collect();
    // let mut current = "AAA";
    // let part1 = instructions
    //     .chars()
    //     .cycle()
    //     .position(|instruction| {
    //         match instruction {
    //             'L' => current = map[current].0,
    //             'R' => current = map[current].1,
    //             _ => unreachable!(),
    //         };
    //         current == "ZZZ"
    //     })
    //     .unwrap()
    //     + 1;

    let current: Vec<_> = map
        .keys()
        .filter(|k: &&&str| k.ends_with('A'))
        .copied()
        .collect();

    let result = current
        .into_iter()
        .map(|x| {
            let (start, repeat, goals) = find_repeat_and_z_visits(x, instructions, &map);

            //assumptions about our input
            assert_eq!(goals.len(), 1);
            assert_eq!(goals[0], repeat - start);

            repeat - start
        })
        .reduce(|a, b| Integer::lcm(&a, &b))
        .unwrap();
    println!("{result}");

    // for (repeat_start, repeat_end, visits) in results {
    //     println!(
    //         "{} {} {} {:?}",
    //         repeat_start,
    //         repeat_end,
    //         (repeat_end - repeat_start) % instructions.len(),
    //         visits
    //     );
    // }

    // println!("8.2 {part2}");
    Ok(())
}
