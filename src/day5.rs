use std::fmt::Debug;
use std::ops::Range;
use std::{fs, str::FromStr};

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space1};
use nom::character::complete::char;
use nom::combinator::map;

use nom::multi::separated_list1;
use nom::sequence::tuple;

#[derive(Debug)]
struct AlmanacMaps {
    maps: Vec<(u64, u64, u64)>
}

impl AlmanacMaps {

    fn translate(&self, from: u64) -> u64 {
        for (dst, src, size) in &self.maps {
            if from >= *src && from < src+size {
                let idx = from - src;
                return dst + idx
            }
        }
        return from;
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: AlmanacMaps,
    soil_to_fertilizer: AlmanacMaps,
    fertilizer_to_water: AlmanacMaps,
    water_to_light: AlmanacMaps,
    light_to_temperature: AlmanacMaps,
    temperature_to_humidity: AlmanacMaps,
    humidity_to_location: AlmanacMaps,
}

impl Almanac {
    fn verbose_translate_seed(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.translate(seed);
        println!("seed to soil: {} -> {}", seed, soil);
        let fertilizer = self.soil_to_fertilizer.translate(soil);
        println!("soil to fertilizer: {} -> {}", soil, fertilizer);
        let water = self.fertilizer_to_water.translate(fertilizer);
        println!("fertilizer to water: {} -> {}", fertilizer, water);
        let light = self.water_to_light.translate(water);
        println!("water to light: {} -> {}", water, light);
        let temperature = self.light_to_temperature.translate(light);
        println!("light to temperature: {} -> {}", light, temperature);
        let humidity = self.temperature_to_humidity.translate(temperature);
        println!("temperature to humidity: {} -> {}", temperature, humidity);
        let location = self.humidity_to_location.translate(humidity);
        println!("humidity to location: {} -> {}", humidity, location);
        location
    }

    fn translate_seed(&self, seed: u64) -> u64 {
        Some(seed)
        .map(|s| self.seed_to_soil.translate(s))
        .map(|s| self.soil_to_fertilizer.translate(s))
        .map(|s| self.fertilizer_to_water.translate(s))
        .map(|s| self.water_to_light.translate(s))
        .map(|s| self.light_to_temperature.translate(s))
        .map(|s| self.temperature_to_humidity.translate(s))
        .map(|s| self.humidity_to_location.translate(s))
        .unwrap()
    }

    fn translate_seeds(&self) -> Vec<u64> {
        self.seeds.iter()
            .map(|s| self.translate_seed(*s))
            .collect()
    }

    fn get_seed_ranges(&self) -> Vec<Range<u64>> {
        self.seeds.iter().tuple_windows().map(|(start, size)| {
            *start..(*start+*size)
        }).collect()
    }

    fn translate_seed_ranges(&self) -> Vec<(u64, u64)> {
        let seed_ranges: Vec<_> = self.seeds.iter().tuple_windows().flat_map(|(start, size)| {
            *start..(*start+*size)
        }).collect();

        println!("{}", &seed_ranges.len());

        seed_ranges.into_iter()
            .map(|s| (self.translate_seed(s), s))
            .collect()
    }

    fn mapline(input: &str) -> nom::IResult<&str, (u64, u64, u64)> {
        map(
            tuple((digit1, space1, digit1, space1, digit1)),
            |(src, _, dst, _, size)| (
                u64::from_str(src).unwrap(),
                u64::from_str(dst).unwrap(),
                u64::from_str(size).unwrap()
            )
        )(input)
    }

    fn parse_seeds_to_soil(input: &str) -> nom::IResult<&str, AlmanacMaps> {
        map(
            tuple((tag("seed-to-soil map:"), newline, separated_list1(newline, Self::mapline), newline, newline)),
            |(_, _, map, _, _)| {
                AlmanacMaps { maps: map.into_iter().sorted_by_key(|(d, _s, _w)| *d).collect() }
            },
        )(input)
    }

    fn parse_soil_to_fertilizer(input: &str) -> nom::IResult<&str, AlmanacMaps> {
        map(
            tuple((tag("soil-to-fertilizer map:"), newline, separated_list1(newline, Self::mapline), newline, newline)),
            |(_, _, map, _, _)| {
                AlmanacMaps { maps: map.into_iter().sorted_by_key(|(d, _s, _w)| *d).collect() }
            },
        )(input)
    }

    fn fertilizer_to_water(input: &str) -> nom::IResult<&str, AlmanacMaps> {
        map(
            tuple((tag("fertilizer-to-water map:"), newline, separated_list1(newline, Self::mapline), newline, newline)),
            |(_, _, map, _, _)| {
                AlmanacMaps { maps: map.into_iter().sorted_by_key(|(d, _s, _w)| *d).collect() }
            },
        )(input)
    }

    fn water_to_light(input: &str) -> nom::IResult<&str, AlmanacMaps> {
        map(
            tuple((tag("water-to-light map:"), newline, separated_list1(newline, Self::mapline), newline, newline)),
            |(_, _, map, _, _)| {
                AlmanacMaps { maps: map.into_iter().sorted_by_key(|(d, _s, _w)| *d).collect() }
            },
        )(input)
    }

    fn light_to_temperature(input: &str) -> nom::IResult<&str, AlmanacMaps> {
        map(
            tuple((tag("light-to-temperature map:"), newline, separated_list1(newline, Self::mapline), newline, newline)),
            |(_, _, map, _, _)| {
                AlmanacMaps { maps: map.into_iter().sorted_by_key(|(d, _s, _w)| *d).collect() }
            },
        )(input)
    }

    fn temperature_to_humidity(input: &str) -> nom::IResult<&str, AlmanacMaps> {
        map(
            tuple((tag("temperature-to-humidity map:"), newline, separated_list1(newline, Self::mapline), newline, newline)),
            |(_, _, map, _, _)| {
                AlmanacMaps { maps: map.into_iter().sorted_by_key(|(d, _s, _w)| *d).collect() }
            },
        )(input)
    }

    fn humidity_to_location(input: &str) -> nom::IResult<&str, AlmanacMaps> {
        map(
            tuple((tag("humidity-to-location map:"), newline, separated_list1(newline, Self::mapline), newline)),
            |(_, _, map, _)| {
                AlmanacMaps { maps: map.into_iter().sorted_by_key(|(d, _s, _w)| *d).collect() }
            },
        )(input)
    }

    fn parse_seeds(input: &str) -> nom::IResult<&str, Vec<u64>> {
        map(
            tuple((tag("seeds"), char(':'), space1, separated_list1(space1, digit1), newline, newline)),
            |(_, _, _, seeds, _, _)| {
                seeds
                    .into_iter()
                    .map(u64::from_str)
                    .map(|x| x.unwrap())
                    .collect()
            },
        )(input)
    }

    fn parse(i: &str) -> nom::IResult<&str, Almanac> {
        map(tuple((
            Self::parse_seeds, 
            Self::parse_seeds_to_soil,
            Self::parse_soil_to_fertilizer,
            Self::fertilizer_to_water,
            Self::water_to_light,
            Self::light_to_temperature,
            Self::temperature_to_humidity,
            Self::humidity_to_location
        )), |(
            seeds,
            seeds_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
        )| Almanac {
            seeds: seeds,
            seed_to_soil: seeds_to_soil,
            soil_to_fertilizer: soil_to_fertilizer,
            fertilizer_to_water: fertilizer_to_water,
            water_to_light: water_to_light,
            light_to_temperature: light_to_temperature,
            temperature_to_humidity: temperature_to_humidity,
            humidity_to_location: humidity_to_location,
        })(i)
    }
}

impl FromStr for Almanac {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Almanac::parse(s)
            .map(|(_r, v)| v)
            .map_err(|e| e.to_owned())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/day5.txt")?;
    let result: Almanac = str::parse(&input)?;

    // println!("{:?}", result);
    println!("{:?}", result.translate_seeds().iter().min().unwrap());

    // println!("{:?}", result.translate_seed_ranges().iter().min().unwrap());
    // let mut min: Option<(u64, u64)> = None;

    // for r in result.get_seed_ranges() {
    //     for s in r {
    //         let l = result.translate_seed(s);
    //         if min == None || l < min.unwrap().0 {
    //             min = Some((l, s));
    //         }
    //     }
    // }
    // let (s, l) = if let Some((s, l)) = min { (s, l)} else { panic!() };
    // println!("{} -> {}", s, l);

    Ok(())
}
