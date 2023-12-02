fn part1(lines: Vec<&str>) -> u32 {
    let p: u32 = lines.iter().map(|x: str| return x);
    return p;
}

#[cfg(test)]
mod tests {
    #[test]
    fn day1_test() {
        use crate::day1;
        let testin = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(day1::part1(testin.lines().collect::<Vec<_>>()), 142)
    }
}
