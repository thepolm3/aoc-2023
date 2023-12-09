use itertools::Itertools;

fn get_next(seq: &[i64]) -> i64 {
    if seq.iter().all_equal() {
        return seq.first().copied().unwrap_or(0);
    }
    let diffs: Vec<_> = seq.iter().tuple_windows().map(|(a, b)| b - a).collect();

    *seq.last().unwrap_or(&0) + get_next(&diffs)
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day9.txt")?;
    let mut input: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<i64>, _>>()
                .unwrap()
        })
        .collect();
    let part1 = input.iter().map(|line| get_next(line)).sum::<i64>();
    println!("{}", part1);

    let part2 = input
        .iter_mut()
        .map(|line| {
            line.reverse();
            get_next(line)
        })
        .sum::<i64>();

    println!("{}", part2);
    Ok(())
}
