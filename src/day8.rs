use std::collections::{HashSet, HashMap};
use std::{error::Error, fs, cmp::Ordering};
use std::fmt::Debug;
use std::fmt;
use itertools::Itertools;
use nom::multi::many1;
use nom::{IResult, multi, InputIter};
use nom::character::complete::{digit0, alpha1, space1, char, newline, alphanumeric1};
use nom::combinator::map;
use nom::sequence::{tuple, pair};

type Label = String;

type T = (String, Vec<(Label, (Label, Label))>);

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input(fs::read_to_string("sample/day8.txt")?)?;
    // println!("{}", solution1(&input)?);
    println!("{}", solution2(&input)?);
    Ok(())
}

fn parse_node(input: &str) -> IResult<&str, (Label, (Label, Label))> {
    map(
        tuple((alphanumeric1::<&str, _>, space1, char('='), space1, char('('), alphanumeric1, char(','), space1, alphanumeric1, char(')'), newline)),
        |(label, _, _, _, _, left, _, _, right, _, _)| (label.to_owned(), (left.to_owned(), right.to_owned()))
    )(input)
}

fn read_input(input: String) -> Result<T, Box<dyn Error>> {
    let (_remainder, x) = map(
        tuple((alpha1, newline, newline, many1(parse_node))),
        |(instructions, _, _, labels)| (instructions.to_owned(), labels)
    )(&input).map_err(|e| e.to_owned())?;
    Ok(x)
}

fn solution1(input: &T) -> Result<u64, Box<dyn Error>> {
    let repeated_instructions = input.0.chars().cycle();
    let mut table = input.1.clone();
    table.sort();
    let mut label = "AAA".to_owned();
    let mut steps = 0;
    // println!("Starting at label AAA");
    for lr in repeated_instructions {
        let jump_to_idx = table.binary_search_by_key(&label, |(lbl, _)| lbl.to_owned()).ok().unwrap();
        let node = &table[jump_to_idx];
        // println!("Found node {:?}", node);
        assert_eq!(label, node.0);
        label = if lr == 'L' {
            node.1.0.clone() 
        } else if lr == 'R' { 
            node.1.1.clone() 
        } else { 
            panic!(); 
        };
        // println!("instruction is {} so new label is {}", lr, label);
        steps += 1;
        if label == "ZZZ" {
            break;
        }
    }
    Ok(steps)
}

fn solution2(input: &T) -> Result<u64, Box<dyn Error>> {
    let repeated_instructions = input.0.chars().enumerate().cycle();
    let mut table = input.1.clone();
    table.sort();
    let labels = table.clone().into_iter().filter(|e| e.0.ends_with("A")).map(|(l, _)| l).collect_vec();

    for orig_label in labels.iter() {
        let mut steps: u64 = 0;
        let mut label = orig_label.clone();
        let mut seen: HashMap<(usize, String), u64> = HashMap::new();
        for (instruction_step, lr) in repeated_instructions.clone() {
            if let Some(v) = seen.insert((instruction_step, label.clone()), steps) {
                if label.ends_with('Z') {
                    println!("for {}, at t={}, we saw ({}, {}) at t={}", orig_label, steps, instruction_step, label, v);
                    break;
                }
            }
            let jump_to_idx = &table.binary_search_by_key(&label, |(lbl, _)| lbl.to_string()).ok().unwrap();
            let (next_label, (left, right)) = &table[*jump_to_idx];
            assert_eq!(label, *next_label);
            let new_label = if lr == 'L' {
                left.clone()
            } else if lr == 'R' {
                right.clone()
            } else {
                panic!()
            };
            println!("for {}, at t={}, we are at {} and will jump to {}", orig_label, steps, label, new_label);
            steps += 1;
            label = new_label;
        }
        // println!("instruction is {} so new label is {}", lr, label);
        if labels.iter().all(|l| l.ends_with("Z")) {
            break;
        }

    }
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        Ok(())
    }


}