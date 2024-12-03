use regex::Regex;

// i probably should have used a lexer at this point - but this still works, for regex reference:
//
// mul\((\d+),(\d+)\)|don't\(\)|do\(\)
//
// matches mul(1234, 5678), do() and don't(), while extracting the digits as match groups.

fn part1(lines: Vec<String>) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(&lines.join(" "))
        .map(|c| {
            (
                c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            )
        })
        .map(|x| x.0 * x.1)
        .sum()
}

fn part2(lines: Vec<String>) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for (op, arg1, arg2) in re
        .captures_iter(&lines.join(" "))
        .map(|c| (c.get(0).unwrap().as_str(), c.get(1), c.get(2)))
    {
        match op {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if enabled {
                    sum += arg1.unwrap().as_str().parse::<usize>().unwrap()
                        * arg2.unwrap().as_str().parse::<usize>().unwrap();
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod day3 {
    use crate::day3::{part1, part2};
    #[test]
    fn test_part1() {
        let t = "xmul(2,4)%&mul[3,7]@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let r = part1(aoc::lines_str(t));
        assert_eq!(r, 161);
    }

    #[test]
    fn test_part1_real() {
        dbg!(part1(aoc::lines_file("./input/day3.txt")));
    }

    #[test]
    fn test_part2() {
        let t = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let r = part2(aoc::lines_str(t));
        assert_eq!(r, 48);
    }

    #[test]
    fn test_part2_real() {
        dbg!(part2(aoc::lines_file("./input/day3.txt")));
    }
}
