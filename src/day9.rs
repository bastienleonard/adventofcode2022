use std::io::BufRead;

pub fn part1() -> i32 {
    let motions = parse_input();
    let mut head_position = Vec2::new(0, 0);
    let mut tail_position = Vec2::new(0, 0);
    let mut visited_tail_positions = std::collections::HashSet::<Vec2>::new();

    for motion in motions.iter() {
        for _ in 0..motion.steps {
            head_position.move_in_direction(motion.direction);
            make_tail_follow_head(&mut tail_position, &mut head_position);
            visited_tail_positions.insert(tail_position);
        }
    }

    visited_tail_positions.len() as i32
}

fn make_tail_follow_head(tail_position: &mut Vec2, head_position: &mut Vec2) {
    if tail_position == head_position {
        return;
    }

    if tail_position.is_next_to(*head_position) {
        return;
    }

    *tail_position += tail_move_vector(*tail_position, *head_position);
}

fn tail_move_vector(tail_position: Vec2, head_position: Vec2) -> Vec2 {
    let mut move_vector = Vec2::new(0, 0);

    if tail_position.x < head_position.x {
        move_vector.x = 1;
    } else if tail_position.x > head_position.x {
        move_vector.x = -1;
    }

    if tail_position.y < head_position.y {
        move_vector.y = 1;
    } else if tail_position.y > head_position.y {
        move_vector.y = -1;
    }

    move_vector
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Copy, Clone, Debug)]
struct TailMotion {
    direction: Direction,
    steps: i32
}

fn parse_input() -> Vec<TailMotion> {
    let file = std::fs::File::open("input9.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap())
        .map(|line| {
             let direction = parse_direction(line.as_bytes()[0]);
            let steps = std::str::from_utf8(&line.as_bytes()[2..])
                .unwrap()
                .parse::<i32>()
                .unwrap();
             TailMotion {
                 direction,
                 steps
             }
        })
        .collect()
}

fn parse_direction(char: u8) -> Direction {
    match char {
        76 => Direction::Left,
        82 => Direction::Right,
        85 => Direction::Up,
        68 => Direction::Down,
        _ => unreachable!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    fn move_in_direction(&mut self, direction: Direction) {
        *self += match direction {
            Direction::Left => Vec2::new(-1, 0),
            Direction::Right => Vec2::new(1, 0),
            Direction::Up => Vec2::new(0, -1),
            Direction::Down => Vec2::new(0, 1)
        };
    }

    fn is_next_to(self, other: Vec2) -> bool {
        [
            Vec2::new(-1, 0),
            Vec2::new(1, 0),
            Vec2::new(0, -1),
            Vec2::new(0, 1),
            Vec2::new(-1, -1),
            Vec2::new(-1, 1),
            Vec2::new(1, -1),
            Vec2::new(1, 1)
        ].iter()
            .any(|&direction| self + direction == other)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl std::ops::AddAssign<Self> for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
