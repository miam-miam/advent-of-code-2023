use std::collections::HashMap;
use itertools::Itertools;
use prse::{Parse, parse, ParseChars, ParseError, try_parse};

#[derive(Parse, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
#[repr(u8)]
enum Card {
    #[prse = "A"]
    Ace = 14,
    #[prse = "K"]
    King = 13,
    #[prse = "Q"]
    Queen = 12,
    #[prse = "J"]
    Jack = 11,
    #[prse = "T"]
    Ten = 10,
    #[prse = "{}"]
    Num(u8) = 9
}


#[derive(Parse, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
#[repr(u8)]
enum Card2 {
    #[prse = "A"]
    Ace = 14,
    #[prse = "K"]
    King = 13,
    #[prse = "Q"]
    Queen = 12,
    #[prse = "J"]
    Joker = 0,
    #[prse = "T"]
    Ten = 10,
    #[prse = "{}"]
    Num(u8) = 9
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: u32,
}

impl<'a> Parse<'a> for Hand {
    fn from_str(s: &'a str) -> Result<Self, ParseError> {
        let (cards, bid) = try_parse!(s, "{::05} {}")?;
        Ok(Self {
            bid,
            cards,
            hand_type: Hand::get_type(cards)
        })
    }
}

impl Hand {
    fn get_type(cards: [Card; 5]) -> HandType {
        let set = cards.iter().counts();
        let (highest_card, highest_count) = set.iter().max_by_key(|s| s.1).unwrap();
        if *highest_count == 5 {
            HandType::FiveOfKind
        } else if *highest_count == 4 {
            HandType::FourOfKind
        } else if *highest_count == 3 && *set.iter().find(|s| s.0 != highest_card).unwrap().1 == 2 {
            HandType::FullHouse
        } else if *highest_count == 3 {
            HandType::ThreeOfKind
        } else if *highest_count == 2 && *set.iter().filter(|s| s.0 != highest_card).max_by_key(|s| s.1).unwrap().1 == 2 {
            HandType::TwoPair
        } else if *highest_count == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut v = input.lines().map(|l| {
        let hand: Hand = parse!(l, "{}");
        hand
    }).collect_vec();
    v.sort();
    Some(v.iter().enumerate().map(|(i, h)| (i + 1) * h.bid as usize).sum())
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Hand2 {
    hand_type: HandType,
    cards: [Card2; 5],
    bid: u32,
}

impl Hand2 {
    fn get_type(cards: [Card2; 5]) -> HandType {
        let set = cards.iter().counts();
        let joker_count = cards.iter().filter(|c| matches!(c, Card2::Joker)).count();
        if set.get(&Card2::Joker).is_some_and(|c| *c == 5) {
            return HandType::FiveOfKind;
        }
        let (highest_card, highest_count) = set.iter().filter(|(c, _)| !matches!(c, Card2::Joker)).max_by_key(|s| s.1).unwrap();

        let highest_count = *highest_count + joker_count;

        if highest_count == 5 {
            HandType::FiveOfKind
        } else if highest_count == 4 {
            HandType::FourOfKind
        } else if highest_count == 3 && *set.iter().find(|(c, _)| *c != highest_card && !matches!(c, Card2::Joker)).unwrap().1 == 2 {
            HandType::FullHouse
        } else if highest_count == 3 {
            HandType::ThreeOfKind
        } else if highest_count == 2 && *set.iter().filter(|(c, _)| *c != highest_card && !matches!(c, Card2::Joker)).max_by_key(|s| s.1).unwrap().1 == 2 {
            HandType::TwoPair
        } else if highest_count == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl<'a> Parse<'a> for Hand2 {
    fn from_str(s: &'a str) -> Result<Self, ParseError> {
        let (cards, bid) = try_parse!(s, "{::05} {}")?;
        Ok(Self {
            bid,
            cards,
            hand_type: Hand2::get_type(cards)
        })
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut v = input.lines().map(|l| {
        let hand: Hand2 = parse!(l, "{}");
        hand
    }).collect_vec();
    v.sort();
    Some(v.iter().enumerate().map(|(i, h)| (i + 1) * h.bid as usize).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(6440));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(5905));
    }
}
