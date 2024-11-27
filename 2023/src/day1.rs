fn part1(lines: Vec<String>) -> u32 {
    lines
        .into_iter()
        .filter_map(|line| {
            let nums = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            Some(nums.first()? * 10 + nums.last()?)
        })
        .sum()
}

fn part2(lines: Vec<String>) -> u32 {
    let pairs = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ];
    part1(
        lines
            .into_iter()
            .map(|l| {
                let mut l = l;
                // this is inefficient and shitty, anyways it works
                pairs.iter().for_each(|pair| {
                    l = l.replace(pair.0, &format!("{}{}{}", pair.0, pair.1, pair.0));
                });
                l
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn day1_test() {
        use crate::day1;
        let testin = String::from(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(day1::part1(aoc::lines_str(&testin)), 142)
    }

    #[test]
    fn day1_test_real() {
        dbg!(day1::part1(aoc::lines_file("input/day1.txt")));
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
        assert_eq!(day1::part2(aoc::lines_str(&testin)), 281)
    }

    #[test]
    fn day1_test_real_part2() {
        dbg!(day1::part2(aoc::lines_file("input/day1.txt")));
    }
}
