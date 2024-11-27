fn part1(lines: Vec<String>) -> u32 {
    lines
        .into_iter()
        .filter_map(|l| {
            let fields = l.split_whitespace().collect::<Vec<&str>>();
            let id = &fields.get(1)?;
            for chunk in fields
                .iter()
                .skip(2)
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .chunks(2)
            {
                let mut counters: (i32, i32, i32) = (12, 14, 13);
                let amount = chunk.first()?.parse::<i32>().ok()?;
                match chunk.last()?.replace(",", "").replace(";", "").as_str() {
                    "red" => counters.0 -= amount,
                    "blue" => counters.1 -= amount,
                    "green" => counters.2 -= amount,
                    _ => return None,
                };
                if counters.0 < 0 || counters.1 < 0 || counters.2 < 0 {
                    return None;
                }
            }

            id.replace(":", "").parse::<u32>().ok()
        })
        .sum()
}

fn part2(lines: Vec<String>) -> u128 {
    lines
        .into_iter()
        .filter_map(|l| {
            let fields = l.split_whitespace().collect::<Vec<&str>>();
            let mut counters = (0, 0, 0);
            for chunk in fields
                .iter()
                .skip(2)
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .chunks(2)
            {
                let amount = chunk.first()?.parse::<u128>().ok()?;
                match chunk.last()?.replace(",", "").replace(";", "").as_str() {
                    "red" => counters.0 = std::cmp::max(counters.0, amount),
                    "blue" => counters.1 = std::cmp::max(counters.1, amount),
                    "green" => counters.2 = std::cmp::max(counters.2, amount),
                    _ => return None,
                };
            }
            Some(counters.0 * counters.1 * counters.2)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day2;

    #[test]
    fn day2_test() {
        let testin = String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(day2::part1(aoc::lines_str(&testin)), 8)
    }

    #[test]
    fn day2_test_real() {
        dbg!(day2::part1(aoc::lines_file("input/day2.txt")));
    }

    #[test]
    fn day2_test_part2() {
        use crate::day2;
        let testin = String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(
            day2::part2(testin.lines().map(String::from).collect()),
            2286
        )
    }

    #[test]
    fn day2_test_real_part2() {
        dbg!(day2::part2(aoc::lines_file("input/day2.txt")));
    }
}
