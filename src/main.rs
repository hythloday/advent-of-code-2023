use std::{error::Error, fs, cmp::Ordering};
use std::fmt::Debug;
use std::fmt;
use itertools::Itertools;

type T = Vec<()>;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input(fs::read_to_string("input/dayX.txt")?)?;
    println!("{}", solution1(&input)?);
    println!("{}", solution2(&input)?);
    Ok(())
}

fn read_input(s: String) -> Result<T, Box<dyn Error>> {
    s.trim().lines().map(|line| {
        todo!()
    }).collect()
}

fn solution1(input: &T) -> Result<u64, Box<dyn Error>> {
    todo!()
}

fn solution2(input: &T) -> Result<u64, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        Ok(())
    }


}