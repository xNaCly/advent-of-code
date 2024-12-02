fn check_integers(integers: Vec<isize>) -> Option<()> {
    let mut last = integers.get(0)?;
    let is_decreasing = integers.iter().skip(1).all(|x| {
        let p = x < last;
        last = x;
        p
    });
    last = integers.get(0)?;
    let is_increasing = integers.iter().skip(1).all(|x| {
        let p = x > last;
        last = x;
        p
    });

    last = integers.get(0)?;
    let has_valid_diff = integers.iter().skip(1).all(|x| {
        let bigger = std::cmp::max(last, x);
        let smaller = std::cmp::min(last, x);
        let diff = bigger - smaller;
        let p = diff >= 1 && diff <= 3;
        last = x;
        p
    });
    ((is_increasing || is_decreasing) && has_valid_diff).then_some(())
}

fn part1(lines: Vec<String>) -> usize {
    lines
        .into_iter()
        .filter_map(|l| {
            let integers = l
                .split_whitespace()
                .filter_map(|num| num.parse::<isize>().ok())
                .collect::<Vec<_>>();
            if integers.is_empty() {
                return None;
            }
            check_integers(integers)
        })
        .count()
}

fn part2(lines: Vec<String>) -> usize {
    lines
        .into_iter()
        .filter_map(|l| {
            let integers = l
                .split_whitespace()
                .filter_map(|num| num.parse::<isize>().ok())
                .collect::<Vec<_>>();
            if integers.is_empty() {
                return None;
            }
            for (idx, _) in integers.iter().enumerate() {
                let mut i = integers.clone();
                i.remove(idx);
                if check_integers(i).is_some() {
                    return Some(());
                }
            }
            None
        })
        .count()
}

#[cfg(test)]
mod day1 {
    use crate::day2::{part1, part2};

    #[test]
    fn test_part1() {
        let t = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let r = part1(aoc::lines_str(t));
        assert_eq!(r, 2);
    }

    #[test]
    fn test_part1_real() {
        dbg!(part1(aoc::lines_file("./input/day2.txt")));
    }

    #[test]
    fn test_part2() {
        let t = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let r = part2(aoc::lines_str(t));
        assert_eq!(r, 4);
    }

    #[test]
    fn test_part2_real() {
        dbg!(part2(aoc::lines_file("./input/day2.txt")));
    }
}
