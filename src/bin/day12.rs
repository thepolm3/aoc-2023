use itertools::Itertools;
use rayon::prelude::*;

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
        //an empty description matches anything without any '#'s
        if state.description.len() <= di {
            let result = !state.pattern[pi..].contains('#') as u64;
            return result;
        }

        let description = &state.description[di..];
        if state.pattern.len() <= pi {
            return 0;
        }
        let pattern = &state.pattern[pi..];
        //now pattern and description are slices of the original pattern and description

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
            result += cached(state, pi + index + 1, di);
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
            result += cached(state, pi + index + first, di + 1);
            return result;
        }

        let next = next.unwrap();
        //if the sequence of '#'s is longer than we expected, we're done
        if next == "#" {
            return result;
        }

        //otherwise, we've found first '#' or '?'s, terminated by a '.' or '?', so we recurse
        result + cached(state, pi + index + first + 1, di + 1)
    }

    fn cached(state: &mut State, x: usize, y: usize) -> u64 {
        if let Some(result) = state.get_cache((x, y)) {
            return result;
        }
        let result: u64 = n_matches_from_index(state, x, y);
        state.set_cache((x, y), result);
        result
    }

    n_matches_from_index(&mut State::new(pattern, description), 0, 0)
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day12.txt")?;
    let [part1, part2] = input
        .par_lines()
        .map(|line| {
            let (pattern, description) = line.split_once(' ').unwrap();
            let description = description
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<usize>, _>>()
                .unwrap();

            let unfolded = std::iter::repeat(pattern).take(5).collect_vec().join("?");
            let repeated = description.repeat(5);
            [
                n_matches(pattern, &description),
                n_matches(&unfolded, &repeated),
            ]
        })
        .reduce(|| [0, 0], |a, b| [a[0] + b[0], a[1] + b[1]]);

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
