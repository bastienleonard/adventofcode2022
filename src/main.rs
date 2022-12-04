use std::io::BufRead;

fn ex1_1() -> i32 {
    let file = std::fs::File::open("input1.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let mut max_calories = -1;
    let mut current_calories = 0;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            if current_calories > max_calories {
                max_calories = current_calories;
            }

            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }

    max_calories
}

fn ex1_2() -> i32 {
    let file = std::fs::File::open("input1.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let mut biggest3: Vec<i32> = vec![];
    let mut current_calories = 0;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            if biggest3.len() == 0 {
                biggest3.push(current_calories);
            } else {
                let mut insertion_index: Option<usize> = None;

                for (index, the_cals) in biggest3.iter_mut().enumerate() {
                    if current_calories > *the_cals {
                        insertion_index = Some(index);
                        break;
                    }
                }

                if let Some(index) = insertion_index {
                    biggest3.insert(index, current_calories);

                    if biggest3.len() == 4 {
                        biggest3.pop();
                    }
                }
            }

            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }

        assert!(biggest3.len() <= 3);
    }

    biggest3.iter().sum()
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

fn ex2_1() -> i32 {
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

fn ex2_2() -> i32 {
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

fn main() {
    assert!(ex1_1() == 65912);
    assert!(ex1_2() == 195625);
    assert!(ex2_1() == 13675);
    assert!(ex2_2() == 14184);
}
