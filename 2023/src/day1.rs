use std::collections::HashMap;

fn part1(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .map(|digits| {
            println!("{:?}", digits);
            (digits.first().unwrap().to_digit(10).unwrap() * 10)
                + digits.last().unwrap().to_digit(10).unwrap()
        })
        .sum()
}

fn part2(lines: Vec<String>) -> u32 {
    let name_to_digit: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    part1(
        lines
            .iter()
            .map(|l| {
                let mut l = l.to_string();
                for (key, value) in name_to_digit.iter() {
                    l = l.replace(key, &format!("{key}{value}{key}"));
                }
                l
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn day1_test() {
        use crate::day1;
        let testin = String::from(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(day1::part1(testin.lines().map(String::from).collect()), 142)
    }

    #[test]
    fn day1_test_real() {
        use crate::day1;
        use std::fs::File;
        use std::io::{self, prelude::*, BufReader};
        let file = File::open("./day01.txt").unwrap();
        let r = BufReader::new(file);
        let lines = r.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        let out = day1::part1(lines);
        println!("result={}", out)
    }

    #[test]
    fn day1_test_part2() {
        use crate::day1;
        let testin = String::from(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(
            day1::part2(
                testin
                    .lines()
                    .filter(|l| !l.is_empty())
                    .map(String::from)
                    .collect()
            ),
            281
        )
    }

    #[test]
    fn day1_test_real_part2() {
        use crate::day1;
        use std::fs::File;
        use std::io::{self, prelude::*, BufReader};
        let file = File::open("./day01.txt").unwrap();
        let r = BufReader::new(file);
        let lines = r.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        let out = day1::part2(lines);
        println!("result={}", out)
    }
}
