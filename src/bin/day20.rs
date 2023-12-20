use anyhow::Context;
use num::{BigInt, Integer};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}
impl Pulse {
    fn invert(self) -> Self {
        use Pulse::*;
        match self {
            High => Low,
            Low => High,
        }
    }
    fn from_bool(b: bool) -> Self {
        match b {
            true => Self::High,
            false => Self::Low,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Gate {
    FlipFlop(Pulse),
    Conjunction(BTreeMap<usize, Pulse>),
    Label,
}
fn main() -> anyhow::Result<()> {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    let input = std::fs::read_to_string("inputs/day20.txt")?;
    let mut labels: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let (label, _) = line.split_once(" -> ")?;

            Some(label.strip_prefix(['%', '&']).unwrap_or(label))
        })
        .collect();
    labels.push("dummy");
    let labels = labels;

    let label_idx = |s| {
        labels
            .iter()
            .position(|&x| x == s)
            .unwrap_or(labels.len() - 1)
    };

    let outputs: Vec<Vec<_>> = input
        .lines()
        .filter_map(|line| {
            let (_, outputs) = line.split_once(" -> ")?;
            Some(outputs.split(", ").map(label_idx).collect())
        })
        .collect();

    let mut gates: Vec<_> = input
        .lines()
        .enumerate()
        .filter_map(|(gate_idx, line)| {
            let (label, _) = line.split_once(" -> ")?;
            Some(match label.chars().next() {
                Some('%') => Gate::FlipFlop(Pulse::Low),
                Some('&') => Gate::Conjunction(
                    outputs
                        .iter()
                        .enumerate()
                        .filter_map(|(i, o)| o.contains(&gate_idx).then_some((i, Pulse::Low)))
                        .collect(),
                ),
                _ => Gate::Label,
            })
        })
        .collect();
    gates.push(Gate::Label);

    let broadcast = label_idx("broadcaster");

    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        low += 1; //button pulse

        //push button
        let mut pulses = VecDeque::from_iter(
            outputs[broadcast]
                .iter()
                .map(|i| (broadcast, *i, Pulse::Low)),
        );
        while let Some((from, to, pulse)) = pulses.pop_front() {
            if pulse == Pulse::High {
                high += 1;
            } else {
                low += 1
            }
            let gate = &mut gates[to];
            match gate {
                Gate::FlipFlop(x) => {
                    if pulse == Pulse::Low {
                        *x = x.invert();
                        pulses.extend(outputs[to].iter().map(|&o| (to, o, *x)));
                    }
                }
                Gate::Conjunction(inputs) => {
                    inputs.insert(from, pulse);
                    let send_pulse =
                        Pulse::from_bool(inputs.iter().all(|(_, &x)| x == Pulse::High)).invert();
                    pulses.extend(outputs[to].iter().map(|&o| (to, o, send_pulse)));
                }
                Gate::Label => {}
            }
        }
    }

    println!("20.1: {}", low * high);

    let goals: Vec<_> = match &gates[label_idx("zh")] {
        Gate::Conjunction(inputs) => inputs,
        _ => panic!(),
    }
    .keys()
    .copied()
    .collect();
    let mut part2 = BigInt::from(1);
    for goal in goals {
        let mut dependencies = HashSet::new();
        let mut new_dependencies = HashSet::new();
        new_dependencies.insert(goal);
        while new_dependencies != dependencies {
            dependencies = new_dependencies.clone();
            for d in &dependencies {
                new_dependencies.extend(
                    outputs
                        .iter()
                        .enumerate()
                        .filter_map(|(i, o)| o.contains(d).then_some(i)),
                );
            }
        }
        println!(
            "{} {:?}",
            labels[goal],
            dependencies.iter().map(|x| labels[*x]).collect::<Vec<_>>()
        );
        let mut cache: HashMap<Vec<_>, _> = HashMap::new();
        let mut presses = 0;
        let (prev, next) = loop {
            presses += 1;
            let mut pulses = VecDeque::from_iter(
                outputs[broadcast]
                    .iter()
                    .map(|i| (broadcast, *i, Pulse::Low)),
            );
            let mut hit_goal = false;
            while let Some((from, to, pulse)) = pulses.pop_front() {
                if to == goal && pulse == Pulse::High {
                    hit_goal = true;
                }
                let gate = &mut gates[to];
                match gate {
                    Gate::FlipFlop(x) => {
                        if pulse == Pulse::Low {
                            *x = x.invert();
                            pulses.extend(outputs[to].iter().map(|&o| (to, o, *x)));
                        }
                    }
                    Gate::Conjunction(inputs) => {
                        inputs.insert(from, pulse);
                        let send_pulse =
                            Pulse::from_bool(inputs.iter().all(|(_, &x)| x == Pulse::High))
                                .invert();
                        pulses.extend(outputs[to].iter().map(|&o| (to, o, send_pulse)));
                    }
                    Gate::Label => {}
                }
            }
            if hit_goal {
                if let Some(prev) = cache.insert(
                    gates
                        .iter()
                        .enumerate()
                        .filter_map(|(i, x)| dependencies.contains(&i).then_some(x.clone()))
                        .collect(),
                    presses,
                ) {
                    break (prev, presses);
                };
            }
        };
        part2 = (part2).lcm(&BigInt::from(next - prev))
    }

    println!("20.2: {part2}");
    Ok(())
}
