use anyhow::Context;
use cached::proc_macro::cached;
use itertools::Itertools;

#[cached]
fn n_matches(pattern: String, description: Vec<usize>) -> u64 {
    //an empty description matches anything without any '#'s
    if description.is_empty() {
        return !pattern.contains('#') as u64;
    }

    //early return if remaining pattern isn't long enough
    if pattern.len() + 1 < description.iter().map(|x| x + 1).sum::<usize>() {
        return 0;
    }

    //the first length of '#'s to find
    let first = description.first().unwrap();

    //index of the first thing that _could_ be a '#'
    let index = pattern.find(['?', '#']);

    //early return if no matches
    if index.is_none() {
        return 0;
    }
    let index = index.unwrap();

    //early return with too short a pattern
    if pattern.len() < *first + index {
        return 0;
    }

    let mut result = 0;

    // principal branch: recurse with '?' is '.'
    if &pattern[index..=index] == "?" {
        result += n_matches(pattern[index + 1..].to_owned(), description.clone());
    }

    //try taking the next "first" characters, they should all be '#' or '?'
    if !(pattern[index..index + first]
        .chars()
        .all(|c| ['?', '#'].contains(&c)))
    {
        return result;
    };

    // the next character after first has matched
    let next = pattern.get((index + first)..=(index + first));

    //if we're at the end of the string, we can return
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

    //otherwise, we've found first '#' or '?'s, terminated by a '.' or '?', so we recurse
    result
        + n_matches(
            pattern[index + first + 1..].to_owned(),
            description[1..].to_owned(),
        )
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
