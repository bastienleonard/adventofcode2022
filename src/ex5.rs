use std::io::BufRead;

pub fn ex5_1() -> String {
    let mut stacks: Vec<Vec<char>> = vec![
        vec!['j', 'h', 'p', 'm', 's', 'f', 'n', 'v'],
        vec!['s', 'r', 'l', 'm', 'j', 'd', 'q'],
        vec!['n', 'q', 'd', 'h', 'c', 's', 'w', 'b'],
        vec!['r', 's', 'c', 'l'],
        vec!['m', 'v', 't', 'p', 'f', 'b'],
        vec!['t', 'r', 'q', 'n', 'c'],
        vec!['g', 'v', 'r'],
        vec!['c', 'z', 's', 'p', 'd', 'l', 'r'],
        vec!['d', 's', 'j', 'v', 'g', 'p', 'b', 'f']
    ];
    let file = std::fs::File::open("input5.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();

    for line in lines {
        let line = line.unwrap();

        if line.starts_with("move") {
            let re = regex::Regex::new(r"move (\d+) from (\d)+ to (\d+)")
                .unwrap();
            assert!(re.is_match(&line));
            let captures = re.captures(&line).unwrap();
            let quantity = captures.get(1).unwrap()
                .as_str()
                .parse::<i32>().unwrap();
            let from = captures.get(2).unwrap()
                .as_str()
                .parse::<i32>().unwrap();
            let to = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            move_crate_1(&mut stacks, quantity, from, to);
        }
    }

    stacks.iter().map(|stack| *stack.last().unwrap())
        .collect::<String>()
        .to_uppercase()
}

fn move_crate_1(
    stacks: &mut Vec<Vec<char>>,
    quantity: i32,
    from: i32, to: i32
) {
    for _ in 0..quantity {
        let crate_ = stacks[(from - 1) as usize].pop().unwrap();
        stacks[(to - 1) as usize].push(crate_);
    }
}

pub fn ex5_2() -> String {
    let mut stacks: Vec<Vec<char>> = vec![
        vec!['j', 'h', 'p', 'm', 's', 'f', 'n', 'v'],
        vec!['s', 'r', 'l', 'm', 'j', 'd', 'q'],
        vec!['n', 'q', 'd', 'h', 'c', 's', 'w', 'b'],
        vec!['r', 's', 'c', 'l'],
        vec!['m', 'v', 't', 'p', 'f', 'b'],
        vec!['t', 'r', 'q', 'n', 'c'],
        vec!['g', 'v', 'r'],
        vec!['c', 'z', 's', 'p', 'd', 'l', 'r'],
        vec!['d', 's', 'j', 'v', 'g', 'p', 'b', 'f']
    ];
    let file = std::fs::File::open("input5.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();

    for line in lines {
        let line = line.unwrap();

        if line.starts_with("move") {
            let re = regex::Regex::new(r"move (\d+) from (\d)+ to (\d+)")
                .unwrap();
            assert!(re.is_match(&line));
            let captures = re.captures(&line).unwrap();
            let quantity = captures.get(1).unwrap()
                .as_str()
                .parse::<i32>().unwrap();
            let from = captures.get(2).unwrap()
                .as_str()
                .parse::<i32>().unwrap();
            let to = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            move_crate_2(&mut stacks, quantity, from, to);
        }
    }

    stacks.iter().map(|stack| *stack.last().unwrap())
        .collect::<String>()
        .to_uppercase()
}

fn move_crate_2(
    stacks: &mut Vec<Vec<char>>,
    quantity: i32,
    from: i32, to: i32
) {
    let mut temp: Vec<char> = vec![];

    for _ in 0..quantity {
        let crate_ = stacks[(from - 1) as usize].pop().unwrap();
        temp.insert(0, crate_);
    }

    for i in 0..temp.len() {
        stacks[(to - 1) as usize].push(temp[i]);
    }
}
