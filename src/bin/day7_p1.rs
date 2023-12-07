use itertools::Itertools;

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    Pair,

    //higher first
    TwoPair,
    ThreeOfAKind,

    //group of 3 first
    FullHouse,

    FourOfAKind,
    FiveOfAKind,
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

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.0 {
            write!(f, "{}", card.0)?;
        }
        Ok(())
    }
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
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

    fn rank(&self) -> HandRank {
        // number of groups of size 2, 3, 4, 5
        let mut groups = [0; 4];
        for (_, group) in &self.0.iter().sorted().group_by(|x| *x) {
            let size = group.count();
            if size > 1 {
                groups[size - 2] += 1;
            }
        }

        match groups {
            [0, 0, 0, 1] => HandRank::FiveOfAKind,
            [0, 0, 1, 0] => HandRank::FourOfAKind,
            [1, 1, 0, 0] => HandRank::FullHouse,
            [0, 1, 0, 0] => HandRank::ThreeOfAKind,
            [2, 0, 0, 0] => HandRank::TwoPair,
            [1, 0, 0, 0] => HandRank::Pair,
            _ => HandRank::HighCard,
        }
    }
}
fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day7.txt")?;

    //     let input = "32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483";

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

    // println!(
    //     "{}",
    //     hands
    //         .iter()
    //         .copied()
    //         .map(|x| format!("{} {:?}", x.0, x.0.rank()))
    //         .join("\n")
    // );

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
        assert_eq!(hand1.rank(), HandRank::Pair);
        assert_eq!(hand2.rank(), HandRank::TwoPair);
        assert_eq!(hand1.rank().cmp(&hand2.rank()), std::cmp::Ordering::Less);
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);

        let hand3 = Hand::from_str("33332").unwrap();
        let hand4 = Hand::from_str("2AAAA").unwrap();
        assert_eq!(hand3.cmp(&hand4), std::cmp::Ordering::Greater);

        assert_eq!(
            Hand::from_str("77888")
                .unwrap()
                .cmp(&Hand::from_str("77788").unwrap()),
            std::cmp::Ordering::Greater
        );

        assert_eq!(
            Hand::from_str("77888")
                .unwrap()
                .cmp(&Hand::from_str("77788").unwrap()),
            std::cmp::Ordering::Greater
        );
    }
}
