#[macro_use]
extern crate maplit;

use std::collections::HashMap;

fn main() {
    let lines = std::fs::read_to_string("src/bin/day01.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
        })
        .expect("Unable to open file");
    let calibration_sum: u32 = lines
        .into_iter()
        .map(|line| get_calibration(line) as u32)
        .sum();
    println!("{}", calibration_sum);
}

fn get_digit(line: &str, start_i: usize, lookup: &HashMap<&str, u16>) -> Option<u16> {
    lookup.iter().find_map(|(needle, val)| {
        if start_i + needle.len() > line.len() {
            None
        } else if &line.get(start_i..start_i + needle.len()).unwrap() == needle {
            Some(*val)
        } else {
            None
        }
    })
}

fn get_calibration(line: String) -> u16 {
    let lookup = hashmap! {
        "one" => 1u16,
        "two" => 2u16,
        "three" => 3u16,
        "four" => 4u16,
        "five" => 5u16,
        "six" => 6u16,
        "seven" => 7u16,
        "eight" => 8u16,
        "nine" => 9u16,
        "1" => 1u16,
        "2" => 2u16,
        "3" => 3u16,
        "4" => 4u16,
        "5" => 5u16,
        "6" => 6u16,
        "7" => 7u16,
        "8" => 8u16,
        "9" => 9u16,
        "0" => 0u16,
    };

    let first = line
        .char_indices()
        .find_map(|(i, _)| get_digit(&line, i, &lookup))
        .unwrap_or_else(|| panic!("could not find digit in {}", line));
    let last = line
        .char_indices()
        .rev()
        .find_map(|(i, _)| get_digit(&line, i, &lookup))
        .unwrap_or_else(|| panic!("could not find digit in {}", line));

    first * 10u16 + last
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_calibration("two1nine".to_string()), 29,)
    }

    #[test]
    fn test2() {
        assert_eq!(get_calibration("eightwothree".to_string()), 83,)
    }

    #[test]
    fn test3() {
        assert_eq!(get_calibration("xtwone3four".to_string()), 24,)
    }

    #[test]
    fn test4() {
        assert_eq!(get_calibration("4nineeightseven2".to_string()), 42,)
    }

    #[test]
    fn test5() {
        assert_eq!(get_calibration("zoneight234".to_string()), 14,)
    }

    #[test]
    fn test6() {
        assert_eq!(get_calibration("7pqrstsixteen".to_string()), 76,)
    }
}
