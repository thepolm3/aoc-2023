use itertools::Itertools;

struct PotentialGear {
    location: (usize, usize),
    adjacent: Vec<u32>,
}
fn is_symbol(x: &u8) -> bool {
    *x != b'.' && !(*x).is_ascii_digit()
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day3.txt")?;
    let lines = input.lines().map(str::as_bytes).collect_vec();

    let mut values_adjacent_to_symbols = Vec::new();

    let mut gears = lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &x)| x == b'*')
                .map(move |(j, _)| PotentialGear {
                    location: (i, j),
                    adjacent: Vec::new(),
                })
        })
        .collect_vec();

    for (i, line) in lines.iter().enumerate() {
        let mut start_idx: Option<usize> = None;

        //add one more byte to cleanup at end of row
        for (j, elem) in line.iter().chain(std::iter::once(&b'.')).enumerate() {
            if elem.is_ascii_digit() {
                if start_idx.is_none() {
                    start_idx = Some(j);
                }
            } else if let Some(range_start) = start_idx {
                let range = (range_start.saturating_sub(1))..=j.min(lines.len() - 1);
                let value = std::str::from_utf8(&line[range_start..j])
                    .expect("all succeeded is_ascii_digit")
                    .parse::<u32>()
                    .expect("sequence of ascii digits is parsable number");

                //part1
                if lines[i.saturating_sub(1)][range.clone()]
                    .iter()
                    .any(is_symbol)
                    || is_symbol(&lines[i][range_start.saturating_sub(1)])
                    || lines[i].get(j).map_or(false, is_symbol)
                    || lines
                        .get(i + 1)
                        .map_or(false, |line| line[range.clone()].iter().any(is_symbol))
                {
                    values_adjacent_to_symbols.push(value);
                }

                //part2
                for gear in gears.iter_mut().filter(|gear| {
                    gear.location.0.abs_diff(i) <= 1
                        && gear.location.1 >= range_start.saturating_sub(1)
                        && gear.location.1 <= j
                }) {
                    gear.adjacent.push(value)
                }

                start_idx = None;
            }
        }
    }
    println!("3.1: {}", values_adjacent_to_symbols.iter().sum::<u32>());
    println!(
        "3.2: {}",
        gears
            .into_iter()
            .map(|gear| gear.adjacent)
            .filter(|x| x.len() == 2)
            .map(|v| v.into_iter().product::<u32>())
            .sum::<u32>()
    );

    Ok(())
}
