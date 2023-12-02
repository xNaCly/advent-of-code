fn part1(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .map(|digits| {
            (digits.first().unwrap().to_digit(10).unwrap() * 10)
                + digits.last().unwrap().to_digit(10).unwrap()
        })
        .sum()
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
        assert_eq!(
            day1::part1(
                testin
                    .lines()
                    .map(|l| String::from(l))
                    .collect::<Vec<String>>()
            ),
            142
        )
    }

    #[test]
    fn day1_test_real() {
        use crate::day1;
        use std::fs::File;
        use std::io::{self, prelude::*, BufReader};
        let file = File::open("./src/input.txt").unwrap();
        let r = BufReader::new(file);
        let lines = r.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        let out = day1::part1(lines);
        println!("result={}", out)
    }
}
