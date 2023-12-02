use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, pair, terminated},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl ColorSet {
    fn new() -> Self {
        ColorSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn set_color(&mut self, col: Color) {
        match col {
            Color::Red(x) => self.red = x,
            Color::Green(x) => self.green = x,
            Color::Blue(x) => self.blue = x,
        }
    }

    fn possible_with(&self, other: ColorSet) -> bool {
        (self.red <= other.red) && (self.blue <= other.blue) && (self.green <= other.green)
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    n: u32,
    sets: Vec<ColorSet>,
}

impl Game {
    fn possible_with(&self, set: ColorSet) -> bool {
        self.sets.iter().all(|s: &ColorSet| s.possible_with(set))
    }
}

fn red(input: &str) -> IResult<&str, Color> {
    map(terminated(digit1, tag(" red")), |x: &str| {
        Color::Red(x.parse::<u32>().unwrap())
    })(input)
}

fn green(input: &str) -> IResult<&str, Color> {
    map(terminated(digit1, tag(" green")), |x: &str| {
        Color::Green(x.parse::<u32>().unwrap())
    })(input)
}

fn blue(input: &str) -> IResult<&str, Color> {
    map(terminated(digit1, tag(" blue")), |x: &str| {
        Color::Blue(x.parse::<u32>().unwrap())
    })(input)
}

fn color(input: &str) -> IResult<&str, Color> {
    alt((red, green, blue))(input)
}

fn color_set(input: &str) -> IResult<&str, ColorSet> {
    let (input, colors) = separated_list0(tag(", "), color)(input)?;

    let mut set = ColorSet::new();
    for color in colors {
        set.set_color(color)
    }

    Ok((input, set))
}

fn game(input: &str) -> IResult<&str, Game> {
    map(
        pair(
            delimited(tag("Game "), digit1, tag(": ")),
            separated_list0(tag("; "), color_set),
        ),
        |(n, sets)| Game {
            n: n.parse::<u32>().unwrap(),
            sets,
        },
    )(input)
}

fn parse(input: &str) -> Vec<Game> {
    input.lines().map(game).map(|x| x.unwrap().1).collect()
}

fn part1(games: &[Game]) -> u32 {
    let set = ColorSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    games
        .iter()
        .filter(|game| game.possible_with(set))
        .map(|g| g.n)
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            game.sets
                .iter()
                .fold(ColorSet::new(), |s1, s2| ColorSet {
                    red: s1.red.max(s2.red),
                    green: s1.green.max(s2.green),
                    blue: s1.blue.max(s2.blue),
                })
                .power()
        })
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day2.txt")?;

    let games = parse(&input);
    println!("2.1: {}", part1(&games));
    println!("2.2: {}", part2(&games));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            8
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            2286
        )
    }
}
