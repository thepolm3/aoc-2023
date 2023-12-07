use anyhow::Context;
use itertools::{izip, Itertools};

fn number_of_winning_moves(total_time: u64, against: u64) -> u64 {
    let half_time = total_time as f64 / 2.;
    let against = against as f64;
    let half_winning_range = (half_time.powi(2) - against).sqrt();

    ((half_time + half_winning_range - 1.).ceil() - (half_time - half_winning_range + 1.).floor())
        as u64
        + 1
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day6.txt")?;

    let (times, victors) = input
        .lines()
        .take(2)
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|x| x.parse::<u64>().expect("Lines consist of integers"))
        })
        .collect_tuple()
        .context("not enough lines")?;

    println!(
        "6.1: {}",
        izip!(times, victors)
            .map(|(time, victor)| number_of_winning_moves(time, victor))
            .product::<u64>()
    );

    let (time, victor) = input
        .lines()
        .take(2)
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .expect("line is a valid integer when whitespace is removed")
        })
        .collect_tuple()
        .expect("Exactly two lines");

    println!("6.2: {}", number_of_winning_moves(time, victor));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_winning_moves() {
        assert_eq!(number_of_winning_moves(7, 9), 4);
        assert_eq!(number_of_winning_moves(15, 40), 8);
        assert_eq!(number_of_winning_moves(30, 200), 9);
        assert_eq!(number_of_winning_moves(71530, 940200), 71503);
    }
}
