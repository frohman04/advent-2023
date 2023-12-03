extern crate regex;

use regex::Regex;
use std::collections::HashMap;

fn main() {
    let lines = std::fs::read_to_string("src/bin/day03.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        })
        .expect("Unable to open file");
    let part_nums = get_part_nums(&lines);
    let symbols = get_symbols(&lines);
    let gear_ratios = get_gear_ratios(part_nums, symbols);
    println!("{:?}", gear_ratios.into_iter().sum::<u32>());
}

fn get_part_nums(lines: &[String]) -> HashMap<(usize, usize), u16> {
    let re = Regex::new("(\\d+)").expect("unable to construct regex");
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_i, line)| {
            re.find_iter(line).flat_map(move |m| {
                (m.start()..m.end()).map(move |row_i| {
                    (
                        (line_i, row_i),
                        m.as_str()
                            .parse::<u16>()
                            .unwrap_or_else(|v| panic!("unable to parse number from {}", v)),
                    )
                })
            })
        })
        .collect()
}

fn get_symbols(lines: &[String]) -> HashMap<(usize, usize), char> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_i, line)| {
            line.char_indices().filter_map(move |(row_i, c)| {
                if c.is_ascii_punctuation() && c != '.' {
                    Some(((line_i, row_i), c))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn get_gear_ratios(
    part_numbers: HashMap<(usize, usize), u16>,
    symbols: HashMap<(usize, usize), char>,
) -> Vec<u32> {
    let mut out = symbols
        .iter()
        .filter_map(|((line_i, row_i), c)| {
            if *c == '*' {
                let part_numbers = get_adjacent_part_numbers(&part_numbers, *line_i, *row_i);
                if part_numbers.len() == 2 {
                    Some(part_numbers[0] as u32 * part_numbers[1] as u32)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<u32>>();
    out.sort();
    out
}

fn get_adjacent_part_numbers(
    part_numbers: &HashMap<(usize, usize), u16>,
    line_i: usize,
    row_i: usize,
) -> Vec<u16> {
    let mut out: Vec<u16> = Vec::new();

    // row above symbol
    out.append(&mut get_vertical_adjacent_part_numbers(
        part_numbers,
        line_i - 1,
        row_i,
    ));

    // left of symbol
    part_numbers
        .get(&(line_i, row_i - 1))
        .iter()
        .for_each(|val| out.push(**val));

    // right of symbol
    part_numbers
        .get(&(line_i, row_i + 1))
        .iter()
        .for_each(|val| out.push(**val));

    // row below symbol
    out.append(&mut get_vertical_adjacent_part_numbers(
        part_numbers,
        line_i + 1,
        row_i,
    ));

    out.sort();
    out
}

fn get_vertical_adjacent_part_numbers(
    part_numbers: &HashMap<(usize, usize), u16>,
    line_i: usize,
    row_i: usize,
) -> Vec<u16> {
    let mut out: Vec<u16> = Vec::new();

    let tl = part_numbers.get(&(line_i, row_i - 1));
    let tc = part_numbers.get(&(line_i, row_i));
    let tr = part_numbers.get(&(line_i, row_i + 1));

    match (tl, tc, tr) {
        (Some(l), Some(_), Some(_)) => out.push(*l),
        (Some(l), Some(_), None) => out.push(*l),
        (None, Some(_), Some(r)) => out.push(*r),
        (Some(l), None, Some(r)) => {
            out.push(*l);
            out.push(*r)
        }
        (Some(l), None, None) => out.push(*l),
        (None, None, Some(r)) => out.push(*r),
        (None, Some(c), None) => out.push(*c),
        (None, None, None) => (),
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn test_get_part_nums() {
        assert_eq!(
            get_part_nums(
                &"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                    .split('\n')
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
            ),
            hashmap! {
                (0, 0) => 467,
                (0, 1) => 467,
                (0, 2) => 467,
                (0, 5) => 114,
                (0, 6) => 114,
                (0, 7) => 114,
                (2, 2) => 35,
                (2, 3) => 35,
                (2, 6) => 633,
                (2, 7) => 633,
                (2, 8) => 633,
                (4, 0) => 617,
                (4, 1) => 617,
                (4, 2) => 617,
                (5, 7) => 58,
                (5, 8) => 58,
                (6, 2) => 592,
                (6, 3) => 592,
                (6, 4) => 592,
                (7, 6) => 755,
                (7, 7) => 755,
                (7, 8) => 755,
                (9, 1) => 664,
                (9, 2) => 664,
                (9, 3) => 664,
                (9, 5) => 598,
                (9, 6) => 598,
                (9, 7) => 598,
            }
        )
    }

    #[test]
    fn test_get_symbols() {
        assert_eq!(
            get_symbols(
                &"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                    .split('\n')
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
            ),
            hashmap! {
                (1, 3) => '*',
                (3, 6) => '#',
                (4, 3) => '*',
                (5, 5) => '+',
                (8, 3) => '$',
                (8, 5) => '*',
            },
        )
    }

    #[test]
    fn test_get_adjacent_part_numbers() {
        assert_eq!(
            get_adjacent_part_numbers(
                &hashmap! {
                    (0, 0) => 467,
                    (0, 1) => 467,
                    (0, 2) => 467,
                    (0, 5) => 114,
                    (0, 6) => 114,
                    (0, 7) => 114,
                    (2, 2) => 35,
                    (2, 3) => 35,
                    (2, 6) => 633,
                    (2, 7) => 633,
                    (2, 8) => 633,
                    (4, 0) => 617,
                    (4, 1) => 617,
                    (4, 2) => 617,
                    (5, 7) => 58,
                    (5, 8) => 58,
                    (6, 2) => 592,
                    (6, 3) => 592,
                    (6, 4) => 592,
                    (7, 6) => 755,
                    (7, 7) => 755,
                    (7, 8) => 755,
                    (9, 1) => 664,
                    (9, 2) => 664,
                    (9, 3) => 664,
                    (9, 5) => 598,
                    (9, 6) => 598,
                    (9, 7) => 598,
                },
                1,
                3,
            ),
            vec![35, 467]
        )
    }

    #[test]
    fn test_get_gear_ratios() {
        assert_eq!(
            get_gear_ratios(
                hashmap! {
                    (0, 0) => 467,
                    (0, 1) => 467,
                    (0, 2) => 467,
                    (0, 5) => 114,
                    (0, 6) => 114,
                    (0, 7) => 114,
                    (2, 2) => 35,
                    (2, 3) => 35,
                    (2, 6) => 633,
                    (2, 7) => 633,
                    (2, 8) => 633,
                    (4, 0) => 617,
                    (4, 1) => 617,
                    (4, 2) => 617,
                    (5, 7) => 58,
                    (5, 8) => 58,
                    (6, 2) => 592,
                    (6, 3) => 592,
                    (6, 4) => 592,
                    (7, 6) => 755,
                    (7, 7) => 755,
                    (7, 8) => 755,
                    (9, 1) => 664,
                    (9, 2) => 664,
                    (9, 3) => 664,
                    (9, 5) => 598,
                    (9, 6) => 598,
                    (9, 7) => 598,
                },
                hashmap! {
                    (1, 3) => '*',
                    (3, 6) => '#',
                    (4, 3) => '*',
                    (5, 5) => '+',
                    (8, 3) => '$',
                    (8, 5) => '*',
                },
            ),
            vec![16345, 451490]
        )
    }
}
