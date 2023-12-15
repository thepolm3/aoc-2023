enum Op {
    Insert(usize),
    Delete,
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |acc, c| (17 * (acc + *c as usize)) % 256)
}
fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day15.txt")?;
    let part1 = input.split(',').map(hash).sum::<usize>();

    let part2 = input
        .split(',')
        .map(|s| {
            if let Some((s, x)) = s.split_once('=') {
                (s, Op::Insert(x.parse::<usize>().unwrap()))
            } else {
                (s.strip_suffix('-').unwrap(), Op::Delete)
            }
        })
        .fold(vec![Vec::new(); 256], |mut hm, (key, op)| {
            let v = &mut hm[hash(key)];
            let i = v.iter().position(|(k, _)| k == &key);
            match op {
                Op::Insert(value) => {
                    if let Some(i) = i {
                        v[i] = (key, value);
                    } else {
                        v.push((key, value));
                    }
                }
                Op::Delete => {
                    if let Some(i) = i {
                        v.remove(i);
                    }
                }
            };
            hm
        })
        .into_iter()
        .enumerate()
        .map(|(i, hm)| {
            (i + 1)
                * hm.into_iter()
                    .enumerate()
                    .map(|(j, (_, v))| v * (j + 1))
                    .sum::<usize>()
        })
        .sum::<usize>();

    println!("15.1: {part1}");
    println!("15.2: {part2}");
    Ok(())
}
