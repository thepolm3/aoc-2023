use anyhow::Result;
use regex::Regex;

fn digit(input: &str) -> &str {
    match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "zero" => "0",
        x => x,
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line: &str| {
            let first = line.chars().find(|x| x.is_digit(10))?;
            let last: char = line.chars().rev().find(|x| x.is_digit(10))?;
            format!("{first}{last}").parse::<u32>().ok()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|zero|[0-9])").unwrap();
    input
        .lines()
        .inspect(|x| println!("{x}"))
        .filter_map(|line| {
            let first = re.find(line)?.as_str();
            let mut idx: usize = line.len() - 1;
            while re.find_at(line, idx) == None {
                idx -= 1;
            }
            let last = re.find_at(line, idx).unwrap().as_str();
            Some(format!("{}{}", digit(first), digit(last)))
        })
        .filter_map(|s| s.parse::<u32>().ok())
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day1.txt")?;

    println!("1.1 {}", part1(&input));

    println!("1.2 {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(part2(input), 281);
    }
}
