use std::fs::File;
use std::io::Read;

pub fn part1() -> i32 {
    let forest = parse_input();

    forest.iter()
        .filter(|ForestIteratorItem { height, x, y }|
                is_tree_visible(&forest, *x, *y, *height))
        .count() as i32
}

pub fn part2() -> i32 {
    let forest = parse_input();

    forest.iter()
        .map(|ForestIteratorItem { height, x, y }|
             tree_score(&forest, x, y, height))
        .max()
        .unwrap()
}

fn is_tree_visible(forest: &Forest, x: i32, y: i32, height: i32) -> bool {
    if is_around_edge(forest, x, y) {
        return true;
    }

    [
        Direction::Left,
        Direction::Right,
        Direction::Top,
        Direction::Bottom
    ].iter()
        .any(|&direction|
             direction_makes_tree_visible(direction, x, y, height, forest))
}

fn direction_makes_tree_visible(
    direction: Direction,
    x: i32,
    y: i32,
    height: i32,
    forest: &Forest
) -> bool {
    let mut x = x;
    let mut y = y;

    loop {
        (x, y) = direction.transform(x, y);

        match forest.get(x, y) {
            Some(neighbor_height) => {
                if neighbor_height >= height {
                    break;
                }
            }
            None => {
                return true;
            }
        }
    }

    false
}

fn is_around_edge(forest: &Forest, x: i32, y: i32) -> bool {
    x == 0 || x == forest.width - 1 || y == 0 || y == forest.height - 1
}

fn tree_score(forest: &Forest, x: i32, y: i32, height: i32) -> i32 {
    [
        Direction::Left,
        Direction::Right,
        Direction::Top,
        Direction::Bottom
    ].iter()
        .map(|&direction| direction_score(direction, x, y, height, forest))
        .product()
}

fn direction_score(
    direction: Direction,
    x: i32,
    y: i32,
    height: i32,
    forest: &Forest
) -> i32 {
    let mut score = 0;
    let mut x = x;
    let mut y = y;

    loop {
        (x, y) = direction.transform(x, y);

        match forest.get(x, y) {
            None => {
                break;
            }
            Some(neighbor_height) => {
                score += 1;

                if neighbor_height >= height {
                    break;
                }
            }
        }
    }

    score
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom
}

impl Direction {
    fn transform(self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Top => (x, y - 1),
            Direction::Bottom => (x, y + 1)
        }
    }
}

#[derive(Debug)]
struct Forest {
    width: i32,
    height: i32,
    trees: Vec<i32>
}

impl Forest {
    fn new(width: i32, height: i32, trees: Vec<i32>) -> Self {
        Self {
            width,
            height,
            trees
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<i32> {
        if !((0..self.width).contains(&x) && (0..self.height).contains(&y)) {
            None
        } else {
            Some(self.trees[(x + y * self.width) as usize])
        }
    }

    fn iter(&self) -> ForestIterator {
        ForestIterator::new(self)
    }
}

struct ForestIterator<'a> {
    forest: &'a Forest,
    x: i32,
    y: i32
}

impl<'a> ForestIterator<'a> {
    fn new(forest: &'a Forest) -> Self {
        Self {
            forest,
            x: 0,
            y: 0
        }
    }
}

impl<'a> Iterator for ForestIterator<'a> {
    type Item = ForestIteratorItem;

    fn next(&mut self) -> Option<ForestIteratorItem> {
        self.forest.get(self.x, self.y)
            .map(|height| {
                let x = self.x;
                let y = self.y;
                self.x += 1;

                if self.x == self.forest.width {
                    self.x = 0;
                    self.y += 1;
                }

                ForestIteratorItem {
                    height,
                    x,
                    y
                }
            })
    }
}

struct ForestIteratorItem {
    height: i32,
    x: i32,
    y: i32
}

impl std::fmt::Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.width {
            for x in 0..self.height {
                write!(f, "{}", self.get(x, y).unwrap())?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_input() -> Forest {
    let mut file = File::open("input8.txt").unwrap();
    let mut trees = Vec::<i32>::new();
    let mut width = 0;
    let mut height = 0;

    loop {
        match read_byte(&mut file) {
            None => {
                break;
            }
            Some(c) => {
                // Newlinw
                if c == 10 {
                    // Even the last line has a newline
                    height += 1;
                } else {
                    if height == 0 {
                        width += 1;
                    }

                    let height = c as i32 - 48;
                    assert!((0..=9).contains(&height));
                    trees.push(height);
                }
            }
        }
    }

    Forest::new(width, height, trees)
}

fn read_byte(file: &mut File) -> Option<u8> {
    let mut byte = [0u8; 1];
    let bytes_read = file.read(&mut byte).unwrap();

    match bytes_read {
        0 => None,
        1 => Some(byte[0]),
        _ => unreachable!()
    }
}
