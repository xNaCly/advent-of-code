use std::collections::HashMap;

fn left_right(lines: Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let mut left = vec![];
    let mut right = vec![];
    lines.into_iter().for_each(|l| {
        let split = l
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
        if split.is_empty() {
            return;
        }
        let raw_nums = (split.first().unwrap(), split.last().unwrap());
        left.push(raw_nums.0.parse::<u64>().unwrap());
        right.push(raw_nums.1.parse::<u64>().unwrap());
    });
    (left, right)
}

fn part1(lines: Vec<String>) -> u64 {
    let (mut left, mut right) = left_right(lines);
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|t| {
            let bigger = std::cmp::max(t.0, t.1);
            let smaller = std::cmp::min(t.0, t.1);
            bigger - smaller
        })
        .sum()
}

fn part2(lines: Vec<String>) -> u64 {
    let mut counter: HashMap<u64, u64> = HashMap::new();
    let (left, right) = left_right(lines);
    for num in right {
        let count = match counter.get(&num) {
            Some(c) => *c + 1,
            _ => 1,
        };
        counter.insert(num, count);
    }
    left.into_iter()
        .map(|n| n * counter.get(&n).unwrap_or(&(0 as u64)))
        .sum()
}

#[cfg(test)]
mod day1 {
    use crate::day1::{part1, part2};

    #[test]
    fn test_part1() {
        let t = r"
3   4
4   3
2   5
1   3
3   9
3   3";
        let r = part1(aoc::lines_str(t));
        assert_eq!(r, 11);
    }

    #[test]
    fn test_part1_real() {
        dbg!(part1(aoc::lines_file("./input/day1.txt")));
    }

    #[test]
    fn test_part2() {
        let t = r"
3   4
4   3
2   5
1   3
3   9
3   3";
        let r = part2(aoc::lines_str(t));
        assert_eq!(r, 31);
    }

    #[test]
    fn test_part2_real() {
        dbg!(part2(aoc::lines_file("./input/day1.txt")));
    }
}
