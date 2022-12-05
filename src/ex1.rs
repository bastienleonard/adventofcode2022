use std::io::BufRead;

pub fn ex1_1() -> i32 {
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

pub fn ex1_2() -> i32 {
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
