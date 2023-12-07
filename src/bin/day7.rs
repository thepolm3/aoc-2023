use itertools::Itertools;

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    Pair(Card),

    //higher first
    TwoPair(Card, Card),
    ThreeOfAKind(Card),

    //group of 3 first
    FullHouse(Card, Card),

    FiveOfAKind(Card),
    FourOfAKind(Card),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card(char);

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        CARDS
            .iter()
            .position(|c| c == &self.0)
            .cmp(&CARDS.iter().position(|c| c == &other.0))
    }
}

impl Card {
    fn from_char(c: char) -> Option<Self> {
        CARDS.contains(&c).then_some(Card(c))
    }
}

//Hand contains a hand of cards, in descending order of value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand([Card; 5]);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

//note this implementation relies of Hand being in descending card order
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.rank(), self.0).cmp(&(other.rank(), other.0))
    }
}
impl Hand {
    fn new(mut cards: [Card; 5]) -> Self {
        //sort in reverse
        cards.sort_unstable_by(|a, b| b.cmp(a));
        Self(cards)
    }
    fn from_str(s: &str) -> Option<Self> {
        let t = s
            .chars()
            .take(5)
            .map(Card::from_char)
            .collect::<Option<Vec<Card>>>()?;

        //try_into should never fail here since we've already got a vec
        //of size 5, with valid cards in it
        Some(Self::new(t.try_into().ok()?))
    }

    //note this implementation relies of Hand being sorted
    fn rank(&self) -> HandRank {
        // number of groups of size 2, 3, 4, 5
        let mut groups = Vec::new();
        for (key, group) in &self.0.iter().group_by(|x| *x) {
            groups.push((group.count(), *key));
        }

        //sort groups high to low by size, then card value
        groups.sort_unstable_by(|a, b| b.cmp(a));

        let mut groups = groups.into_iter();
        match [groups.next(), groups.next()] {
            [Some((5, c)), None] => HandRank::FiveOfAKind(c),
            [Some((4, c)), _] => HandRank::FourOfAKind(c),
            [Some((3, c1)), Some((2, c2))] => HandRank::FullHouse(c1, c2),
            [Some((3, c)), _] => HandRank::ThreeOfAKind(c),
            [Some((2, c1)), Some((2, c2))] => HandRank::TwoPair(c1, c2),
            [Some((2, c)), _] => HandRank::Pair(c),
            _ => HandRank::HighCard,
        }
    }
}
fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day7.txt")?;

    let mut hands = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').expect("Line consists of hand and bid");
            (
                Hand::from_str(cards).expect("Hand is valid hand"),
                bid.parse::<u32>().expect("Bid is integer"),
            )
        })
        .collect_vec();

    hands.sort();

    let part1 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * (*bid))
        .sum::<u32>();

    println!("7.1: {part1}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hand_cmp() {
        let hand1 = Hand::from_str("32T3K").unwrap();
        let hand2 = Hand::from_str("KK677").unwrap();
        assert_eq!(hand1.rank(), HandRank::Pair(Card('3')));
        assert_eq!(hand2.rank(), HandRank::TwoPair(Card('K'), Card('7')));
        assert_eq!(hand1.rank().cmp(&hand2.rank()), std::cmp::Ordering::Less);
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);
    }
}
