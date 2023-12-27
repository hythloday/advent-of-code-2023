use std::{error::Error, fs, str::FromStr};

use nom::bytes::complete::*;
use nom::combinator::complete;
use nom::multi::*;
use nom::sequence::*;
use nom::{character::complete::*, combinator::*, IResult};

#[derive(Clone, Debug, PartialEq)]
struct Card {
    id: usize,
    winning: Vec<u32>,
    have: Vec<u32>,
    matches: usize,
    score: u32,
}

impl Card {
    fn new(id: usize, winning: Vec<u32>, have: Vec<u32>) -> Card {
        let matches = have.iter().filter(|h| winning.contains(h)).count();
        Card {
            id: id,
            winning: winning,
            have: have,
            matches: matches,
            score: if matches == 0 { 0 } else { 1 << matches - 1 },
        }
    }

    fn parse_number(input: &str) -> IResult<&str, u32> {
        map_res(digit1, u32::from_str)(input)
    }

    fn parse_number_list(c: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(space1, Self::parse_number)(c)
    }

    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    fn parse_card(c: &str) -> IResult<&str, Card> {
        let line_parser = tuple((
            tag("Card"),
            space1,
            Self::parse_number,
            char(':'),
            space1,
            Self::parse_number_list,
            space1,
            char('|'),
            space1,
            Self::parse_number_list,
        ));
        map(line_parser, |(_, _, id, _, _, winning, _, _, _, have)| {
            Card::new(id as usize, winning, have)
        })(c)
    }
}

impl FromStr for Card {

    // Result<_, nom::Err<(String, nom::Err<String>)>>
    // Result<_, nom::Err<nom::error::Error<String>>>

    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        complete(Card::parse_card)(s)
            .map(|(_, c)| c)
            .map_err(|e| e.to_owned())
    }
}

// not memoizing this is fast enough
fn transitive_value_of_card(card: &Card, set: &Vec<Card>) -> u32 {
    1 + (card.id..card.id + card.matches)
        .map(|c| transitive_value_of_card(&set[c], set))
        .sum::<u32>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day4.txt")?;
    let result: Result<Vec<Card>, _> = input.lines().map(str::parse).collect();
    let cards = result?;

    let sum: u32 = cards.clone().iter().map(|c| c.score).sum();
    println!("{}", sum);

    let total_cards: u32 = cards
        .iter()
        .map(|c: &Card| transitive_value_of_card(c, &cards))
        .sum();
    println!("{}", total_cards);
    Ok(())
}

#[cfg(test)]
mod tests_p1 {
    use crate::*;

    #[test]
    fn test_parse_card() -> Result<(), Box<dyn Error>> {
        let card_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card: Card = card_str.parse()?;
        let expected: Card = Card::new(
            1,
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53],
        );
        assert_eq!(card, expected);
        assert_eq!(card.score, 8);
        Ok(())
    }

    #[test]
    fn test_parse_cards() -> Result<(), Box<dyn Error>> {
        let cards_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards: Result<Vec<Card>, _> = cards_str
            .lines()
            .map(|s| s.trim())
            .map(str::parse)
            .collect();
        let expected: Vec<Card> = vec![
            Card::new(1, vec!(41, 48, 83, 86, 17), vec!(83, 86, 6, 31, 17, 9, 48, 53)),
            Card::new(2, vec!(13, 32, 20, 16, 61), vec!(61, 30, 68, 82, 17, 32, 24, 19)),
            Card::new(3, vec!(1, 21, 53, 59, 44), vec!(69, 82, 63, 72, 16, 21, 14, 1)),
            Card::new(4, vec!(41, 92, 73, 84, 69), vec!(59, 84, 76, 51, 58, 5, 54, 83)), 
            Card::new(5, vec!(87, 83, 26, 28, 32), vec!(88, 30, 70, 12, 93, 22, 82, 36)),
            Card::new(6, vec!(31, 18, 13, 56, 72), vec!(74, 77, 10, 23, 35, 67, 36, 11))
        ];
        assert_eq!(cards, Ok(expected));
        assert_eq!(cards?.iter().map(|c| c.score).sum::<u32>(), 13);
        Ok(())
    }

    #[test]
    fn test_transitive_value() -> Result<(), Box<dyn Error>> {
        let cards_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result: Result<Vec<Card>, _> = cards_str
            .lines()
            .map(|s| s.trim())
            .map(str::parse)
            .collect();
        let cards = result?;
        let total_cards = cards
            .iter()
            .map(|c: &Card| transitive_value_of_card(c, &cards))
            .sum::<u32>();
        assert_eq!(total_cards, 30);
        Ok(())
    }
}
