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

fn get_calibration(line: String) -> u16 {
    let first_i = line
        .find(|c: char| c.is_ascii_digit())
        .unwrap_or_else(|| panic!("could not find digit in {}", line));
    let last_i = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
    let first = line.chars().nth(first_i).unwrap().to_digit(10).unwrap() as u16;
    let last = line.chars().nth(last_i).unwrap().to_digit(10).unwrap() as u16;
    first * 10u16 + last
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_calibration("1abc2".to_string()), 12,)
    }

    #[test]
    fn test2() {
        assert_eq!(get_calibration("pqr3stu8vwx".to_string()), 38,)
    }

    #[test]
    fn test3() {
        assert_eq!(get_calibration("a1b2c3d4e5f".to_string()), 15,)
    }

    #[test]
    fn test4() {
        assert_eq!(get_calibration("treb7uchet".to_string()), 77,)
    }
}
