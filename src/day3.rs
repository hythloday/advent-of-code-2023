use std::error::Error;
use std::fs;
use regex::Regex;
use std::str::FromStr;


fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day3.txt")?;
    let lines: Vec<_> = input.lines().collect();

    let (numbers_boxes, symbols_boxes) = get_parts_and_symbols(lines.clone());
    let symboled_numbers: Vec<_> = numbers_boxes.iter().filter(|numbox| get_symbol_neighbours(numbox, &symbols_boxes).len() > 0).collect();
    let machine_part_sum: u32 = symboled_numbers.iter().map(|numbox| numbox.value).sum();
    println!("{}", machine_part_sum);

    let gears = symbols_boxes.iter()
        .filter(|s| s.chr == '*')
        .filter(|s| get_number_neighbours(s, &symboled_numbers).len() == 2);

    let ratios: Vec<Vec<_>> = gears.map(|g| get_number_neighbours(&g, &symboled_numbers).iter().map(|n| n.value).collect())
        .collect();

    let ratio_sum: u32 = ratios.iter().map(|v| { let p: u32 = v.iter().product(); p }).sum();

    println!("{:?}", ratio_sum);

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Point {
    chr: char,
    x: usize,
    y: usize
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NumberBox {
    numbox: (Point, Point),
    value: u32,
}

fn get_parts_and_symbols(lines: Vec<&str>) -> (Vec<NumberBox>, Vec<Point>) {

    let symbols: Vec<_> = lines.iter().enumerate().flat_map(|(l, line)| 
        line.trim().chars().enumerate().filter_map(move| (c, chr)| 
            if chr != '.' && !chr.is_digit(10) {
                Some(Point {chr: chr, x: c, y: l})
            } else { None }
        )
    ).collect();

    let re = Regex::new(r"([0-9]+)").unwrap();

    let numboxes: Vec<_> = lines.iter().enumerate().flat_map(|(l, line)|
        re.find_iter(line.trim()).map(move |mtch| NumberBox {
            numbox: (Point {chr: ' ', x: mtch.start(), y: l}, Point {chr: ' ', x: mtch.end() - 1, y: l}), value: u32::from_str(mtch.as_str()).unwrap()
        })
    ).collect();

    (numboxes, symbols)
}

fn get_symbol_neighbours<'a>(numbox: &&NumberBox, symbols: &'a Vec<Point>) -> Vec<&'a Point> {
    symbols.iter().filter(|symbol| {
        symbol.x as i32 >= (numbox.numbox.0.x as i32) - 1 && symbol.x <= numbox.numbox.1.x + 1 &&
        symbol.y as i32 >= (numbox.numbox.0.y as i32) - 1 && symbol.y <= numbox.numbox.1.y + 1
    }).collect()
}

fn get_number_neighbours<'a>(symbol: &&Point, numboxes: &'a Vec<&NumberBox>) -> Vec<&'a &'a NumberBox> {
    numboxes.iter().filter(|numbox| {
        symbol.x as i32 >= (numbox.numbox.0.x as i32) - 1 && symbol.x <= numbox.numbox.1.x + 1 &&
        symbol.y as i32 >= (numbox.numbox.0.y as i32) - 1 && symbol.y <= numbox.numbox.1.y + 1
    }).collect()
}

#[cfg(test)]
mod tests_p1 {

    use crate::{get_parts_and_symbols, get_symbol_neighbours, Point, NumberBox, get_number_neighbours};

    #[test]
    fn test_example_sum() -> Result<(), String> {
        let s = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..".trim();
        let (parts, symbs) = get_parts_and_symbols(s.lines().collect());

        assert!(!parts.is_empty());
        assert_eq!(parts.len(), 10);
        assert_eq!(symbs.len(), 6);

        assert_eq!(symbs, [
            Point{chr: '*', x:3, y:1},
            Point{chr: '#', x:6, y:3},
            Point{chr: '*', x:3, y:4},
            Point{chr: '+', x:5, y:5},
            Point{chr: '$', x:3, y:8},
            Point{chr: '*', x:5, y:8}
        ]);

        {
            let regression_467 = parts.iter().filter(|p| p.value == 467).next().unwrap();
            assert_eq!(regression_467.numbox, (Point{chr: ' ', x:0, y:0}, Point{chr: ' ', x:2, y:0}));
            assert_eq!(get_symbol_neighbours(&regression_467, &symbs), vec!(&Point{chr: '*', x:3,y:1}));
        }

        {
            let regression_755 = parts.iter().filter(|p| p.value == 755).next().unwrap();
            assert_eq!(regression_755.numbox, (Point{chr: ' ', x:6, y:7}, Point{chr: ' ', x:8, y:7}));
            let expected: Vec<&Point> = vec!(&Point{chr: '*', x:5, y:8});
            assert_eq!(crate::get_symbol_neighbours(&regression_755, &symbs), expected);
        }
        let kept: Vec<_> = parts.iter().filter(|part| get_symbol_neighbours(part, &symbs).len() > 0).collect();
        {
            let expected = vec!(
                &NumberBox { numbox: (Point { chr: ' ', x: 0, y: 0 }, Point { chr: ' ', x: 2, y: 0 }), value: 467 }, 
                &NumberBox { numbox: (Point { chr: ' ', x: 2, y: 2 }, Point { chr: ' ', x: 3, y: 2 }), value: 35 }, 
                &NumberBox { numbox: (Point { chr: ' ', x: 6, y: 2 }, Point { chr: ' ', x: 8, y: 2 }), value: 633 }, 
                &NumberBox { numbox: (Point { chr: ' ', x: 0, y: 4 }, Point { chr: ' ', x: 2, y: 4 }), value: 617 }, 
                &NumberBox { numbox: (Point { chr: ' ', x: 2, y: 6 }, Point { chr: ' ', x: 4, y: 6 }), value: 592 }, 
                &NumberBox { numbox: (Point { chr: ' ', x: 6, y: 7 }, Point { chr: ' ', x: 8, y: 7 }), value: 755 }, 
                &NumberBox { numbox: (Point { chr: ' ', x: 1, y: 9 }, Point { chr: ' ', x: 3, y: 9 }), value: 664 },
                &NumberBox { numbox: (Point { chr: ' ', x: 5, y: 9 }, Point { chr: ' ', x: 7, y: 9 }), value: 598 });
            assert_eq!(kept, expected);
        }

        {
            let machine_part_sum: u32 = kept.iter().map(|numbox| numbox.value).sum();
            assert_eq!(machine_part_sum, 4361);
        }

        {
            let gears: Vec<_> = symbs.iter()
                .filter(|s| s.chr == '*')
                .filter(|s| get_number_neighbours(s, &kept).len() == 2)
                .collect();

            let expected: Vec<&Point> = vec!(
                &Point { chr: '*', x: 3, y: 1 },
                &Point { chr: '*', x: 5, y: 8 }
            );
            assert_eq!(gears, expected);

            let ratios: Vec<Vec<_>> = gears.iter()
                .map(|g| get_number_neighbours(g, &kept).iter().map(|n| n.value).collect())
                .collect();

            assert_eq!(ratios, vec!(vec!(467, 35), vec!(755, 598)));

            let ratio_sum: u32 = ratios.iter().map(|v| { let p: u32 = v.iter().product(); p }).sum();

            assert_eq!(ratio_sum, 467835);

        }


        Ok(())
    }
}
