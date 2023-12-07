use std::{cmp::Reverse, marker::PhantomData};

use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy)]
struct StandardDeck;
#[derive(PartialEq, Eq, Clone, Copy)]
struct JokerDeck;

trait DeckType {
    fn deck() -> [char; 13];
}

impl DeckType for StandardDeck {
    fn deck() -> [char; 13] {
        [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]
    }
}

impl DeckType for JokerDeck {
    fn deck() -> [char; 13] {
        [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ]
    }
}

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
struct Card<DeckType> {
    inner: char,
    _deck_type: PhantomData<DeckType>,
}

impl<T: DeckType + std::cmp::Eq> PartialOrd for Card<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: DeckType + std::cmp::Eq> Ord for Card<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        T::deck()
            .iter()
            .position(|c| c == &self.inner)
            .cmp(&T::deck().iter().position(|c| c == &other.inner))
    }
}

impl<T: DeckType> Card<T> {
    fn from_char(c: char) -> Option<Self> {
        T::deck().contains(&c).then_some(Card {
            inner: c,
            _deck_type: PhantomData,
        })
    }
}

impl From<Card<StandardDeck>> for Card<JokerDeck> {
    fn from(value: Card<StandardDeck>) -> Self {
        Card {
            inner: value.inner,
            _deck_type: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand<T>([Card<T>; 5]);

impl PartialOrd for Hand<StandardDeck> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Hand<JokerDeck> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<StandardDeck> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.rank(), &self.0).cmp(&(other.rank(), &other.0))
    }
}

impl Ord for Hand<JokerDeck> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.rank(), &self.0).cmp(&(other.rank(), &other.0))
    }
}

impl<T> std::fmt::Display for Hand<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.0 {
            write!(f, "{}", card.inner)?;
        }
        Ok(())
    }
}

impl Hand<StandardDeck> {
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

impl Hand<JokerDeck> {
    fn rank(&self) -> HandRank {
        let mut jokers = 0;

        let mut groups = Vec::with_capacity(5);
        for (key, group) in &self.0.iter().sorted().group_by(|x| *x) {
            let size = group.count();
            if key.inner == 'J' {
                jokers += size;
            } else {
                groups.push(size);
            }
        }

        //sort groups high to low by group size, then card value
        groups.sort_unstable_by_key(|x| Reverse(*x));

        //may fail if there are 5 jokers
        if let Some(t) = groups.get_mut(0) {
            *t += jokers
        } else {
            debug_assert_eq!(jokers, 5);
            groups = vec![5];
        }

        let mut groups = groups.into_iter();
        match [groups.next(), groups.next()] {
            [Some(5), None] => HandRank::FiveOfAKind,
            [Some(4), _] => HandRank::FourOfAKind,
            [Some(3), Some(2)] => HandRank::FullHouse,
            [Some(3), _] => HandRank::ThreeOfAKind,
            [Some(2), Some(2)] => HandRank::TwoPair,
            [Some(2), _] => HandRank::Pair,
            _ => HandRank::HighCard,
        }
    }
}

impl<T: DeckType + std::cmp::Eq> Hand<T> {
    fn new(cards: [Card<T>; 5]) -> Self {
        Self(cards)
    }
    fn from_str(s: &str) -> Option<Self> {
        let t = s
            .chars()
            .take(5)
            .map(Card::<T>::from_char)
            .collect::<Option<Vec<_>>>()?;

        //try_into should never fail here since we've already got a vec
        //of size 5, with valid cards in it
        Some(Self::new(t.try_into().ok()?))
    }
}

impl From<Hand<StandardDeck>> for Hand<JokerDeck> {
    fn from(value: Hand<StandardDeck>) -> Self {
        Hand(value.0.map(From::from))
    }
}

fn total_bids<T>(hands: &[(Hand<T>, u32)]) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * (*bid))
        .sum::<u32>()
}
fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day7.txt")?;

    let mut hands: Vec<(Hand<StandardDeck>, u32)> = input
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
    println!("7.1: {}", total_bids(&hands));

    let mut hands: Vec<(Hand<JokerDeck>, u32)> = hands
        .into_iter()
        .map(|(hand, bid)| (hand.into(), bid))
        .collect();

    hands.sort();

    println!("7.2: {}", total_bids(&hands));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hand_cmp() {
        let hand1 = Hand::<StandardDeck>::from_str("32T3K").unwrap();
        let hand2 = Hand::<StandardDeck>::from_str("KK677").unwrap();
        assert_eq!(hand1.rank(), HandRank::Pair);
        assert_eq!(hand2.rank(), HandRank::TwoPair);
        assert_eq!(hand1.rank().cmp(&hand2.rank()), std::cmp::Ordering::Less);
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);

        let hand3 = Hand::<StandardDeck>::from_str("33332").unwrap();
        let hand4 = Hand::<StandardDeck>::from_str("2AAAA").unwrap();
        assert_eq!(hand3.cmp(&hand4), std::cmp::Ordering::Greater);

        assert_eq!(
            Hand::<StandardDeck>::from_str("77888")
                .unwrap()
                .cmp(&Hand::from_str("77788").unwrap()),
            std::cmp::Ordering::Greater
        );

        assert_eq!(
            Hand::<StandardDeck>::from_str("77888")
                .unwrap()
                .cmp(&Hand::from_str("77788").unwrap()),
            std::cmp::Ordering::Greater
        );
    }
}
