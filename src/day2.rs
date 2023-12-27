extern crate nom;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{pair, tuple};
use nom::IResult;
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day2.txt")?;
    let lines = input.lines().clone();
    let game_parses_result: Result<Vec<Game>, _> = lines
        .map(parse_game_and_id)
        .into_iter()
        .map(|x| x.map(|(_, game)| game))
        .collect();
    let game_parses = game_parses_result.unwrap();

    {
        let bag_max = Draw {
            r: 12, g: 13, b: 14
        };
        let game_id_sums: u32 = game_parses.clone().iter()
            .filter(|g| g.draws.iter().all(|d| d.r <= bag_max.r && d.g <= bag_max.g && d.b <= bag_max.b))
            .map(|g| g.id)
            .sum();
        println!("{}", game_id_sums);
    }

    {
        let game_mins: u64 = game_parses.iter()
            .map(|g| {
                let min_r = g.draws.iter().map(|d| d.r).max().unwrap();
                let min_g = g.draws.iter().map(|d| d.g).max().unwrap();
                let min_b = g.draws.iter().map(|d| d.b).max().unwrap();
                (min_r, min_g, min_b)
            })
            .map(|(x, y, z)| (x*y*z) as u64)
            .sum();
        println!("{}", game_mins);
    }

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Draw {
    r: u32,
    g: u32,
    b: u32,
}

pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

fn parse_game_and_id(g: &str) -> nom::IResult<&str, Game> {
    let game_parser = tuple((
        tag("Game"),
        space1,
        parse_numbers,
        char(':'),
        space1,
        separated_list1(pair(char(';'), space1), parse_draw),
    ));

    map(game_parser, |(_, _, b, _, _, draws)| Game {
        id: b,
        draws: draws,
    })(g)
}

fn parse_draw(g: &str) -> nom::IResult<&str, Draw> {
    let draw_parser = separated_list1(
        pair(char(','), space1),
        tuple((
            parse_numbers,
            space1,
            alt((tag("blue"), tag("red"), tag("green"))),
        )),
    );

    map(draw_parser, |draws| {
        draws.iter().fold(
            Draw { r: 0, g: 0, b: 0 },
            |d, (count, _, colour)| match colour {
                &"red" => Draw {
                    r: *count,
                    g: d.g,
                    b: d.b,
                },
                &"blue" => Draw {
                    r: d.r,
                    g: d.g,
                    b: *count,
                },
                &"green" => Draw {
                    r: d.r,
                    g: *count,
                    b: d.b,
                },
                _ => panic!(),
            },
        )
    })(g)
}

#[cfg(test)]
mod tests_p1 {
    use crate::Draw;

    #[test]
    fn test_parse_draw() -> Result<(), String> {
        // let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let s = "3 blue, 4 red";
        let result = crate::parse_draw(s);

        assert!(result.is_ok());
        let (remainder, draw) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(draw, Draw { r: 4, g: 0, b: 3 });

        Ok(())
    }

    #[test]
    fn parse_one_game() -> Result<(), String> {
        let s = "Game 17: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = crate::parse_game_and_id(s);

        assert!(result.is_ok());
        let (remainder, game) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(game.id, 17);
        assert_eq!(
            game.draws,
            vec!(
                Draw { r: 4, g: 0, b: 3 },
                Draw { r: 1, g: 2, b: 6 },
                Draw { r: 0, g: 2, b: 0 }
            )
        );

        Ok(())
    }
}
