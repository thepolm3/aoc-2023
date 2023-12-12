use anyhow::Context;
use itertools::Itertools;

struct State<'a> {
    pattern: &'a str,
    description: &'a [usize],
    cache: Vec<Option<u64>>,
    width: usize,
}
impl<'a> State<'a> {
    fn new(pattern: &'a str, description: &'a [usize]) -> Self {
        Self {
            pattern,
            description,
            cache: vec![None; pattern.len() * description.len()],
            width: pattern.len(),
        }
    }
    fn get_cache(&self, (x, y): (usize, usize)) -> Option<u64> {
        *self.cache.get(self.width * y + x).unwrap_or(&None)
    }
    fn set_cache(&mut self, (x, y): (usize, usize), value: u64) -> Option<u64> {
        self.cache
            .get_mut(y * self.width + x)
            .and_then(|c| c.replace(value))
    }
}

fn n_matches(pattern: &str, description: &[usize]) -> u64 {
    fn n_matches_from_index(state: &mut State<'_>, pi: usize, di: usize) -> u64 {
        //cache hit
        if let Some(result) = state.get_cache((pi, di)) {
            return result;
        }

        //an empty description matches anything without any '#'s
        if state.description.len() <= di {
            let result = !state.pattern[pi..].contains('#') as u64;
            state.set_cache((pi, di), result);
            return result;
        }

        let description = &state.description[di..];
        if state.pattern.len() <= pi {
            state.set_cache((pi, di), 0);
            return 0;
        }
        let pattern = &state.pattern[pi..];
        //now pattern and description are slices of the original pattern and description

        //early return if remaining pattern isn't long enough
        if pattern.len() + 1 < description.iter().map(|x| x + 1).sum::<usize>() {
            state.set_cache((pi, di), 0);
            return 0;
        }

        //the first length of '#'s to find
        let first = description.first().unwrap();

        //index of the first thing that _could_ be a '#'
        let index = pattern.find(['?', '#']);

        //early return if no matches
        if index.is_none() {
            state.set_cache((pi, di), 0);
            return 0;
        }
        let index = index.unwrap();

        //early return with too short a pattern
        if pattern.len() < *first + index {
            state.set_cache((pi, di), 0);
            return 0;
        }

        let mut result = 0;

        // principal branch: recurse with '?' is '.'
        if &pattern[index..=index] == "?" {
            result += n_matches_from_index(state, pi + index + 1, di);
        }

        //try taking the next "first" characters, they should all be '#' or '?'
        if !(pattern[index..index + first]
            .chars()
            .all(|c| ['?', '#'].contains(&c)))
        {
            state.set_cache((pi, di), result);
            return result;
        };

        // the next character after first has matched
        let next = pattern.get((index + first)..=(index + first));

        //if we're at the end of the string, we can return
        if next.is_none() {
            result += n_matches_from_index(state, pi + index + first, di + 1);
            state.set_cache((pi, di), result);
            return result;
        }

        let next = next.unwrap();
        //if the sequence of '#'s is longer than we expected, we're done
        if next == "#" {
            state.set_cache((pi, di), result);
            return result;
        }

        //otherwise, we've found first '#' or '?'s, terminated by a '.' or '?', so we recurse
        result += n_matches_from_index(state, pi + index + first + 1, di + 1);
        state.set_cache((pi, di), result);
        result
    }

    n_matches_from_index(&mut State::new(pattern, description), 0, 0)
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

        part1 += n_matches(pattern, &description);
        part2 += n_matches(&unfolded, &repeated);
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
        assert_eq!(n_matches("???.###", &[1, 1, 3]), 1);
        assert_eq!(n_matches(".??..??...?##.", &[1, 1, 3]), 4);
    }
}
