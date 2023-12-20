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

fn broadcast_pulses<'a>(
    gates: &'_ HashMap<&'_ str, (Gate, Vec<&'a str>)>,
) -> VecDeque<(&'a str, &'a str, Pulse)> {
    gates["broadcaster"]
        .1
        .iter()
        .map(|i| ("broadcast", *i, Pulse::Low))
        .collect()
}

//gets number of low and high signals sent
fn buttonpress_low_hi(gates: &mut HashMap<&str, (Gate, Vec<&str>)>) -> (usize, usize) {
    let mut low = 1; //button pulse
    let mut high = 0;

    //push button
    let mut pulses = broadcast_pulses(gates);
    while let Some((from, to, pulse)) = pulses.pop_front() {
        if pulse == Pulse::High {
            high += 1;
        } else {
            low += 1
        }
        if let Some(gate) = gates.get_mut(to) {
            pulses.extend(send_pulse(gate, pulse, from, to))
        }
    }

    (low, high)
}

//gets number of low and high signals sent
fn buttonpress_pulls_high(gates: &mut HashMap<&str, (Gate, Vec<&str>)>, target: &str) -> bool {
    //push button
    let mut pulses = broadcast_pulses(gates);

    let mut hi_target = false;
    while let Some((from, to, pulse)) = pulses.pop_front() {
        if to == target && pulse == Pulse::High {
            hi_target = true;
        }
        if let Some(gate) = gates.get_mut(to) {
            pulses.extend(send_pulse(gate, pulse, from, to))
        }
    }

    hi_target
}

fn send_pulse<'a>(
    gate: &mut (Gate, Vec<&'a str>),
    pulse: Pulse,
    from: &'a str,
    to: &'a str,
) -> Vec<(&'a str, &'a str, Pulse)> {
    let (gate, outputs) = gate;
    match gate {
        Gate::FlipFlop(x) => {
            if pulse == Pulse::Low {
                *x = x.invert();
                return outputs.iter().map(|&o| (to, o, *x)).collect();
            }
        }
        Gate::Conjunction(inputs) => {
            inputs.insert(from.into(), pulse);
            let send_pulse =
                Pulse::from_bool(inputs.iter().all(|(_, &x)| x == Pulse::High)).invert();
            return outputs.iter().map(|&o| (to, o, send_pulse)).collect();
        }
        Gate::Label => {}
    }
    Vec::new()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Gate {
    FlipFlop(Pulse),
    Conjunction(BTreeMap<String, Pulse>),
    Label,
}
fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day20.txt")?;
    let mut gates: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (label, outputs) = line.split_once(" -> ").expect("Invalid line");
            (
                label.strip_prefix(['%', '&']).unwrap_or(label),
                (
                    match label.chars().next() {
                        Some('%') => Gate::FlipFlop(Pulse::Low),
                        Some('&') => Gate::Conjunction(Default::default()),
                        _ => Gate::Label,
                    },
                    outputs.split(", ").collect::<Vec<_>>(),
                ),
            )
        })
        .collect();

    //seed conjunction initial state
    let gates_copy = gates.clone();
    for (label, (gate, _)) in &mut gates {
        if let Gate::Conjunction(inputs) = gate {
            inputs.extend(
                gates_copy
                    .iter()
                    .filter_map(|(k, (_, o))| {
                        o.contains(label).then_some((k.to_string(), Pulse::Low))
                    })
                    .collect::<Vec<_>>(),
            )
        }
    }
    let gates2 = gates.clone();

    let (low, high) = (0..1000)
        .map(|_| buttonpress_low_hi(&mut gates))
        .fold((0, 0), |(a, b), (x, y)| (a + x, b + y));

    println!("20.1: {}", low * high);

    let penultimate = gates2
        .iter()
        .find(|(_, (_, o))| o.contains(&"rx"))
        .expect("no rx node")
        .0;

    let goals: Vec<_> = match &gates2[penultimate].0 {
        Gate::Conjunction(inputs) => inputs,
        _ => panic!("Penultimate node not a conjunction"),
    }
    .keys()
    .cloned()
    .collect();

    let mut part2 = BigInt::from(1);
    for goal in goals {
        let mut dependencies = HashSet::new();
        let mut new_dependencies = HashSet::new();
        new_dependencies.insert(&goal[..]);
        while new_dependencies != dependencies {
            dependencies = new_dependencies.clone();
            for d in &dependencies {
                new_dependencies.extend(
                    gates
                        .iter()
                        .filter_map(|(label, (_, o))| o.contains(&&d[..]).then_some(label)),
                );
            }
        }

        let mut cache: HashMap<Vec<_>, _> = HashMap::new();
        let mut gates = gates2.clone();

        let mut presses = 0;
        let (prev, next) = loop {
            presses += 1;

            if buttonpress_pulls_high(&mut gates, &goal) {
                if let Some(prev) = cache.insert(
                    gates
                        .iter()
                        .filter_map(|(i, x)| dependencies.contains(&i[..]).then_some(x.clone()))
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
