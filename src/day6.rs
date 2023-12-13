use std::error::Error;
use std::fs;

use core::iter::Iterator;

use itertools::Itertools;

fn parse_input(lines: &mut dyn Iterator<Item=&str>) -> Result<Vec<(u64, u64)>, Box<dyn Error>> {
    let times: Result<Vec<u64>, _> = lines.next().to_owned().unwrap().split_whitespace().skip(1).map(|u| u64::from_str_radix(u, 10)).collect();
    let dist: Result<Vec<u64>, _> = lines.next().to_owned().unwrap().split_whitespace().skip(1).map(|u| u64::from_str_radix(u, 10)).collect();
    Ok(times?.into_iter().zip(dist?.into_iter()).collect())
    // Ok(vec![])
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day6.txt")?;
    let mut lines = input.lines();
    // println!("{:?}", parse_input_1(&mut lines)?);
    println!("{}", solution1(parse_input(&mut lines.clone())?));
    println!("{}", solution2(parse_input(&mut lines)?)?);
    Ok(())
}

fn solution1(times_and_distances: Vec<(u64, u64)>) -> u64 {
    let winning_ways = times_and_distances.into_iter().map(|(t,d)| 
        (0..t).filter(|x| x * (t-x) > d).count()
    );
    println!("{:?}", winning_ways.clone().collect_vec());
    winning_ways.product::<usize>() as u64

}

fn solution2(times_and_distances: Vec<(u64, u64)>) -> Result<u64, Box<dyn Error>> {
    let times = u64::from_str_radix(&times_and_distances.iter().map(|x| x.0.to_string()).join(""), 10)?; 
    let dists = u64::from_str_radix(&times_and_distances.iter().map(|x| x.1.to_string()).join(""), 10)?; 

    let winning_ways =(0..times).filter(|x| x * (times-x) > dists).count();

    Ok(winning_ways as u64)
}
