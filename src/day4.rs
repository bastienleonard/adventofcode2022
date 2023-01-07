use std::io::BufRead;
use std::ops::RangeInclusive;

pub fn part1() -> i32 {
    let file = std::fs::File::open("input4.txt").unwrap();
    std::io::BufReader::new(file).lines()
        .map(|line| {
            let line = line.unwrap();
            let groups: Vec<&str> = line.split(',').collect();
            assert!(groups.len() == 2);
            let first = parse_range(groups[0]);
            let second = parse_range(groups[1]);
            one_fully_contains(first, second) as i32
        }).sum()
}

fn parse_range(s: &str) -> RangeInclusive<i32> {
    let parts: Vec<&str> = s.split('-').collect();
    assert!(parts.len() == 2);
    let start: i32 = parts[0].parse::<i32>().unwrap();
    let end: i32 = parts[1].parse::<i32>().unwrap();
    let range = RangeInclusive::new(start, end);
    assert!(range.end() >= range.start());
    range
}

fn one_fully_contains(
    first: RangeInclusive<i32>,
    second: RangeInclusive<i32>
) -> bool {
    (first.start() <= second.start() && first.end() >= second.end())
        || (second.start() <= first.start() && second.end() >= first.end())
}

pub fn part2() -> i32 {
    let file = std::fs::File::open("input4.txt").unwrap();
    std::io::BufReader::new(file).lines()
        .map(|line| {
            let line = line.unwrap();
            let groups: Vec<&str> = line.split(',').collect();
            assert!(groups.len() == 2);
            let mut first = parse_range(groups[0]);
            let second = parse_range(groups[1]);
            first.any(|i| second.contains(&i)) as i32
        })
        .sum()
}
