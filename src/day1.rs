use std::error::Error;
use std::fs;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day1.txt")?;
    let lines = input.lines();
    println!("{}", add_all_lines(lines));
    Ok(())
}

fn add_all_lines<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
    lines.map(get_both_digits).sum()
}

fn find_numberishes(string: &str) -> Vec<(usize, u8)> {
    let numberishes: &[(&str, u8)] = &[
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ];
    numberishes
        .iter()
        .flat_map(|(p, val)| string.match_indices(p).map(move |(idx, _)| (idx, val)))
        .map(|(x, y)| (x, *y))
        .collect()
}

fn get_first_digit(string: &str) -> u8 {
    find_numberishes(string)
        .iter()
        .sorted_by_key(|(x, _)| x)
        .next()
        .unwrap()
        .1
}

fn get_last_digit(string: &str) -> u8 {
    find_numberishes(string)
        .iter()
        .sorted_by_key(|(x, _)| x)
        .next_back()
        .unwrap()
        .1
}

fn get_both_digits(string: &str) -> u32 {
    10 * get_first_digit(string) as u32 + get_last_digit(string) as u32
}

#[cfg(test)]
mod tests_p1 {
    use crate::add_all_lines;
    use crate::get_both_digits;
    use crate::get_first_digit;
    use crate::get_last_digit;

    fn test_add_case_1st(s: &str, expected: u8) -> Result<(), String> {
        let result = get_first_digit(s);
        if result != expected {
            Err(format!("{} result: {}, expected: {}", s, result, expected))
        } else {
            Ok(())
        }
    }

    fn test_add_case_last(s: &str, expected: u8) -> Result<(), String> {
        let result = get_last_digit(s);
        if result != expected {
            Err(format!("{} result: {}, expected: {}", s, result, expected))
        } else {
            Ok(())
        }
    }

    fn test_add_case_both(s: &str, expected: u32) -> Result<(), String> {
        let result = get_both_digits(s);
        if result != expected {
            Err(format!("{} result: {}, expected: {}", s, result, expected))
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_get_first_digit() -> Result<(), String> {
        [
            ("1abc2", 1),
            ("pqr3stu8vwx", 3),
            ("a1b2c3d4e5f", 1),
            ("treb7uchet", 7),
        ]
        .iter()
        .try_for_each(|(s, expected)| test_add_case_1st(*s, *expected))?;

        Ok(())
    }

    #[test]
    fn test_get_last_digit() -> Result<(), String> {
        [
            ("1abc2", 2),
            ("pqr3stu8vwx", 8),
            ("a1b2c3d4e5f", 5),
            ("treb7uchet", 7),
        ]
        .iter()
        .try_for_each(|(s, expected)| test_add_case_last(*s, *expected))?;

        Ok(())
    }

    #[test]
    fn test_get_both_digits() -> Result<(), String> {
        [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ]
        .iter()
        .try_for_each(|(s, expected)| test_add_case_both(*s, *expected))?;

        Ok(())
    }

    #[test]
    fn test_get_all_lines_product() -> Result<(), String> {
        let s = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let expected = 142;
        let result = add_all_lines(s.lines());
        if result != expected {
            Err(format!("{} result: {}, expected: {}", s, result, expected))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests_p2 {
    use crate::add_all_lines;
    use crate::get_both_digits;

    fn test_add_case_both(s: &str, expected: u32) -> Result<(), String> {
        let result = get_both_digits(s);
        if result != expected {
            Err(format!("{} result: {}, expected: {}", s, result, expected))
        } else {
            Ok(())
        }
    }

    #[test]
    fn regression_73eight7() -> Result<(), String> {
        let s = "73eight7";
        let expected = 77;
        let result = get_both_digits(s);
        if result != expected {
            Err(format!("{} result: {}, expected: {}", s, result, expected))
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_get_both_digits() -> Result<(), String> {
        [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ]
        .iter()
        .try_for_each(|(s, expected)| test_add_case_both(*s, *expected))?;

        Ok(())
    }

    #[test]
    fn test_get_all_lines_product() -> Result<(), String> {
        let s = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let expected = 281;
        let result = add_all_lines(s.lines());
        if result != expected {
            Err(format!("{} result: {}, expected: {}", s, result, expected))
        } else {
            Ok(())
        }
    }
}
