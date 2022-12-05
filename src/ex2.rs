use std::io::BufRead;

pub fn ex2_1() -> i32 {
    let file = std::fs::File::open("input2.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    lines.map(
        |result| {
            let line = result.unwrap();
            let line_bytes = line.as_bytes();
            let line_items = (line_bytes[0] as char, line_bytes[2] as char);
            let prediction = Prediction {
                his_choice: match line_items.0 {
                    'A' => Choice::Rock,
                    'B' => Choice::Paper,
                    'C' => Choice::Scissors,
                    _ => unreachable!()
                },
                my_choice: match line_items.1 {
                    'X' => Choice::Rock,
                    'Y' => Choice::Paper,
                    'Z' => Choice::Scissors,
                    _ => unreachable!()
                }
            };
            result_points(&prediction) + choice_points(&prediction.my_choice)
        }
    ).sum()
}

pub fn ex2_2() -> i32 {
    let file = std::fs::File::open("input2.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    lines.map(
        |result| {
            let line = result.unwrap();
            let line_bytes = line.as_bytes();
            let line_items = (line_bytes[0] as char, line_bytes[2] as char);
            let his_choice = match line_items.0 {
                'A' => Choice::Rock,
                'B' => Choice::Paper,
                'C' => Choice::Scissors,
                _ => unreachable!()
            };
            let game_result = match line_items.1 {
                'X' => GameResult::Loss,
                'Y' => GameResult::Draw,
                'Z' => GameResult::Win,
                _ => unreachable!()
            };
            let my_choice = match game_result {
                GameResult::Win => Choice::winner_against(his_choice),
                GameResult::Loss => Choice::loser_against(his_choice),
                GameResult::Draw => his_choice
            };
            game_result.points() + choice_points(&my_choice)
        }
    ).sum()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

impl Choice {
    fn winner_against(choice: Self) -> Self {
        match choice {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock
        }
    }

    fn loser_against(choice: Self) -> Self {
        match choice {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Prediction {
    his_choice: Choice,
    my_choice: Choice
}

enum GameResult {
    Win,
    Loss,
    Draw
}

impl GameResult {
    fn points(self) -> i32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0
        }
    }
}

fn result_points(prediction: &Prediction) -> i32 {
    if prediction.my_choice == prediction.his_choice {
        GameResult::Draw
    } else if prediction.my_choice == Choice::winner_against(prediction.his_choice) {
        GameResult::Win
    } else {
        GameResult::Loss
    }.points()
}

fn choice_points(choice: &Choice) -> i32 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3
    }
}
