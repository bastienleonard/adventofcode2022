use std::io::BufRead;

pub fn part1() -> i32 {
    let file = std::fs::File::open("input3.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();

    lines.map(
        |line| {
            let line = line.unwrap();
            assert!(line.len() % 2 == 0);
            let (first, last) = line.split_at(line.len() / 2);
            assert!(first.len() == last.len());
            let c = common_char(first, last);
            item_priority(c)
        }
    ).sum()
}

fn common_char(first: &str, last: &str) -> char {
    let mut result = None::<char>;

    for c in first.as_bytes() {
        let c = *c as char;

        if last.find(c).is_some() {
            assert!(result.is_none() || result.unwrap() == c);
            result = Some(c);
        }
    }

    result.unwrap()
}

fn item_priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        (c as i32) - ('a' as i32) + 1
    } else if c.is_ascii_uppercase() {
        (c as i32) - ('A' as i32) + 27
    } else {
        unreachable!()
    }
}

pub fn part2() -> i32 {
    let file = std::fs::File::open("input3.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let mut group: Vec<String> = vec![];
    let mut result = 0;

    for line in lines {
        let line = line.unwrap();
        group.push(line);

        if group.len() == 3 {
            result += item_priority(group_badge(&group));
            group.clear();
        }
    }

    result
}

fn group_badge(group: &[String]) -> char {
    assert!(group.len() == 3);

    for c in group[0].as_bytes() {
        let c = *c as char;

        if group[1].find(c).is_some() && group[2].find(c).is_some() {
            return c;
        }
    }

    unreachable!()
}
