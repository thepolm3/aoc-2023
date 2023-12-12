use anyhow::Context;
use cached::proc_macro::cached;
use itertools::Itertools;

#[cached]
fn n_matches(pattern: String, description: Vec<usize>) -> u64 {
    if description.is_empty() {
        return !pattern.contains('#') as u64;
    }
    if pattern.len() + 1 < description.iter().map(|x| x + 1).sum::<usize>() {
        return 0;
    }
    let first = description.first().unwrap();

    let index = pattern.find(['?', '#']);

    if index.is_none() {
        return 0;
    }

    let index = index.unwrap();

    if pattern.len() < *first + index {
        return 0;
    }

    let mut result = 0;
    if &pattern[index..=index] == "?" {
        // let the ? be .
        result += n_matches(pattern[index + 1..].to_owned(), description.clone());
    }

    //if sequence isn't valid, then we have no valid continuations if ? is a #
    if !(pattern[index..index + first]
        .chars()
        .all(|c| ['?', '#'].contains(&c)))
    {
        return result;
    };

    let next = pattern.get((index + first)..=(index + first));
    if next.is_none() {
        result += n_matches(
            pattern[index + first..].to_owned(),
            description[1..].to_owned(),
        );
        return result;
    }
    let next = next.unwrap();
    //if the sequence of '#'s is longer than we expected, we're done
    if next == "#" {
        return result;
    }

    result += n_matches(
        pattern[index + first + 1..].to_owned(),
        description[1..].to_owned(),
    );
    result
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day12.txt")?;
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let (pattern, description) = line.split_once(' ').context("Invalid line")?;
        let description = description
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()?;

        let unfolded = std::iter::repeat(pattern).take(5).collect_vec().join("?");
        let repeated = description.repeat(5);

        part1 += n_matches(pattern.to_owned(), description);
        part2 += n_matches(unfolded, repeated);
    }
    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matches() {
        assert_eq!(n_matches("???.###".to_owned(), vec![1, 1, 3]), 1);
        assert_eq!(n_matches(".??..??...?##.".to_owned(), vec![1, 1, 3]), 4);
    }
}
