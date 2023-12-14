use std::{error::Error, fs, cmp::Ordering};
use std::fmt::Debug;
use std::fmt;
use itertools::Itertools;

#[derive(Clone, Eq, PartialEq, PartialOrd)] 
struct Hand {
    cards: String,
    points: u64
}

impl Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Hand")
         .field(&self.cards)
         .field(&self.typ_with_wildcard('J'))
         .field(&self.points)
         .finish()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {

    fn cmp_with_jokers(&self, other: &Hand) -> Ordering {

        let c = if self.typ_with_wildcard('J') > other.typ_with_wildcard('J') {
            Ordering::Greater
        } else if self.typ_with_wildcard('J') < other.typ_with_wildcard('J') {
            Ordering::Less
        } else {
            Hand::cmp_first_card_with_jokers(&self.cards, &other.cards)
        };
        c
    }

    fn cmp_first_card(left: &String, right: &String) -> Ordering {
        let cmps = left.chars().zip(right.chars()).map(|(c1, c2)| Hand::card_value(c1).cmp(&Hand::card_value(c2)));
        cmps.filter(|c| *c != Ordering::Equal).nth(0).unwrap()
    }

    fn card_value_with_jokers(card: char) -> u8 {
        if card.is_numeric() {
            u8::from_str_radix(&card.to_string(), 10).unwrap()
        } else {
            match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => panic!(),
            }
        }
    }

    fn cmp_first_card_with_jokers(left: &String, right: &String) -> Ordering {
        let cmps = left.chars().zip(right.chars()).map(|(c1, c2)| Hand::card_value_with_jokers(c1).cmp(&Hand::card_value_with_jokers(c2)));
        cmps.filter(|c| *c != Ordering::Equal).nth(0).unwrap()
    }

    fn card_value(card: char) -> u8 {
        if card.is_numeric() {
            u8::from_str_radix(&card.to_string(), 10).unwrap()
        } else {
            match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => panic!(),
            }
        }
    }

    fn typ_with_wildcard(&self, wildcard: char) -> HandType {
        use HandType::*;

        let jokers = self.cards.chars().filter(|c| *c == wildcard).count();

        let mut lengths = self.cards.chars()
            .filter(|c| *c != wildcard)
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .map(|(_c, g)| g.count())
            .sorted()
            .rev();

        let mut binding = lengths.next();
        let (fst, snd) = (binding.get_or_insert(0), lengths.next());

        match (*fst + jokers, snd) {
            (5, _) => FiveOfAKind,
            (4, _) => FourOfAKind,
            (3, Some(2)) => FullHouse,
            (3, _) => ThreeOfAKind,
            (2, Some(2)) => TwoPair,
            (2, _) => OnePair,
            _ => HighCard
        }
    }

    fn typ(&self) -> HandType {
        // never find this in a deck
        self.typ_with_wildcard('*')
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.typ() > other.typ() {
            Ordering::Greater
        } else if self.typ() < other.typ() {
            Ordering::Less
        } else {
            Hand::cmp_first_card(&self.cards, &other.cards)
        }        
    }
}

type T = Vec<Hand>;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input(fs::read_to_string("input/day7.txt")?)?;
    println!("{}", solution1(&input)?);
    println!("{}", solution2(&input)?);
    Ok(())
}


fn read_input(s: String) -> Result<T, Box<dyn Error>> {
    s.trim().lines().map(|line| {
        if let Some((cards, points)) = line.split_whitespace().next_tuple() {
            Ok(Hand { cards: cards.to_owned(), points: u64::from_str_radix(points, 10).unwrap() })
        } else {
            Err("foo")?
        }
    }).collect()
}

fn solution1(input: &T) -> Result<u64, Box<dyn Error>> {
    Ok(input.clone().into_iter().sorted_by(Hand::cmp).enumerate().map(|(rank, hand)| (rank+1) as u64 * hand.points).sum::<u64>())
}

fn solution2(input: &T) -> Result<u64, Box<dyn Error>> {
    println!("{:?}", (input.clone().into_iter().sorted_by(Hand::cmp_with_jokers).enumerate().collect_vec()));

    Ok(input.clone().into_iter().sorted_by(Hand::cmp_with_jokers).enumerate().map(|(rank, hand)| (rank+1) as u64 * hand.points).sum::<u64>())
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_pair_sort() -> Result<(), Box<dyn Error>> {
        let h0 = Hand { cards: "32T3K".to_owned(), points: 1 };
        let h1 = Hand { cards: "KTJJT".to_owned(), points: 1 };
        let h2 = Hand { cards: "KK677".to_owned(), points: 1 };
        let h3 = Hand { cards: "T55J5".to_owned(), points: 1 };
        let h4 = Hand { cards: "QQQJA".to_owned(), points: 1 };

        assert_eq!(Ordering::Less, h0.cmp(&h1));
        assert_eq!(Ordering::Less, h1.cmp(&h2));
        assert_eq!(Ordering::Less, h2.cmp(&h3));
        assert_eq!(Ordering::Less, h3.cmp(&h4));

        Ok(())
    }
        
    #[test]
    fn test_sample_ordering() -> Result<(), Box<dyn Error>> {
        let mut hands = read_input(fs::read_to_string("sample/day7.txt")?)?;
        hands.sort_by(Hand::cmp);

        assert_eq!(hands[0].cards, "32T3K"); 
        assert_eq!(hands[1].cards, "KTJJT"); 
        assert_eq!(hands[2].cards, "KK677"); 
        assert_eq!(hands[3].cards, "T55J5"); 
        assert_eq!(hands[4].cards, "QQQJA");

        Ok(())
    }

    #[test]
    fn test_joker_sorting() -> Result<(), Box<dyn Error>> {
        let h1 = Hand { cards: "JJJJJ".to_owned(), points: 287 };
        let h2 = Hand { cards: "Q385K".to_owned(), points: 7 };
        assert_eq!(Ordering::Greater, h1.cmp_with_jokers(&h2));

        Ok(())
    }

}