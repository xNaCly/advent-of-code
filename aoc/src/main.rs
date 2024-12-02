use clap::Parser;
use reqwest::Request;
use std::{fs, io::Write, path::PathBuf};

const TEMPLATE: &'static str = r#"
fn part1(lines: Vec<String>) -> usize {
    0
}

fn part2(lines: Vec<String>) -> usize {
    0
}

#[cfg(test)]
mod day$DAYHERE$ {
    use crate::day$DAYHERE$::{part1, part2};
    #[test]
    fn test_part1() {
        let t = "";
        let r = part1(aoc::lines_str(t));
        assert_eq!(r, 0);
    }

    // #[test]
    // fn test_part1_real() {
    //     dbg!(part1(aoc::lines_file("./input/day$DAYHERE$.txt")));
    // }

    // #[test]
    // fn test_part2() {
    //     let t = r"";
    //     let r = part2(aoc::lines_str(t));
    //     assert_eq!(r, 0);
    // }

    // #[test]
    // fn test_part2_real() {
    //     dbg!(part2(aoc::lines_file("./input/day$DAYHERE$.txt")));
    // }
}"#;

/// aoc is a cli for downloading input files and creating structures in the advent of code context
#[derive(Debug, clap::Parser)]
struct Config {
    #[arg(short, long, default_value = "2024")]
    year: String,
    #[arg(short, long, default_value = ".cookie")]
    cookie_file: String,
}

fn new_day(dir: &PathBuf) -> Option<usize> {
    let mut days = dir
        .read_dir()
        .ok()?
        .filter_map(|x| {
            x.ok()?
                .file_name()
                .as_os_str()
                .to_str()?
                .split(".")
                .nth(0)?
                .chars()
                .skip(3)
                .collect::<String>()
                .parse::<usize>()
                .ok()
        })
        .collect::<Vec<_>>();
    days.sort();
    days.last().map(|x| x.clone() + 1)
}

fn create_day(day: usize, dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let dir = dir.join("src");
    let mut oo = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dir.join(format!("day{}.rs", day)))?;
    oo.write_all(TEMPLATE.replace("$DAYHERE$", &day.to_string()).as_bytes())?;
    fs::OpenOptions::new()
        .append(true)
        .open(dir.join("lib.rs"))?
        .write(format!("mod day{};", day).as_bytes())?;
    Ok(())
}

fn load_cookie(cookie_file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(cookie_file_path)?
        .lines()
        .nth(0)
        .ok_or("failed to read the cookie")?
        .to_string())
}

fn download_input(
    cookie: &str,
    url: &str,
    destination: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(destination.parent().ok_or("failed to get parent")?)?;
    let client = reqwest::blocking::Client::new();
    let res = client
        .request(reqwest::Method::GET, url)
        .header("cookie", format!("session={}", cookie))
        .send()?;
    let mut oo = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(destination)?;
    Ok(oo.write_all(&res.bytes()?.to_vec())?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Config::parse();

    let dir: PathBuf = ["./", &args.year].into_iter().collect();

    let day = new_day(&dir.join("src")).unwrap_or(1);
    println!("aoc: found day {}, creating new day {}", day - 1, day);

    println!("aoc: reading cookie from {}", &args.cookie_file);
    let cookie = load_cookie(&args.cookie_file)?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", &args.year, day);
    let destination: PathBuf = [
        dir.to_str().ok_or("failed to get string from dir")?,
        "input",
        &format!("day{}.txt", day),
    ]
    .iter()
    .collect();
    println!(
        "aoc: downloading puzzle input from {:?} into {:?}",
        &url, &destination
    );
    if let Err(err) = download_input(&cookie, &url, &destination) {
        println!("failed to download the input: {}", err);
    }
    if let Err(err) = create_day(day, &dir) {
        println!("failed to create the source file: {}", err);
    }
    Ok(())
}
