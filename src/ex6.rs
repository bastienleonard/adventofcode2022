use std::collections::VecDeque;
use std::io::Read;

pub fn ex6_1() -> i32 {
    let mut file = std::fs::File::open("input6.txt").unwrap();
    let mut last_4_chars: VecDeque<u8> = VecDeque::new();
    let mut chars_read = 0;

    loop {
        let mut byte = [0 as u8; 1];
        let bytes_read = file.read(&mut byte).unwrap();

        if bytes_read == 0 {
            unreachable!();
        }

        chars_read += 1;
        last_4_chars.push_back(byte[0]);

        if last_4_chars.len() == 5 {
            last_4_chars.pop_front();
        }

        if last_4_chars.len() == 4 && unique(&last_4_chars) {
            return chars_read;
        }
    }
}

fn unique(chars: &VecDeque<u8>) -> bool {
    for i in 0..(chars.len() - 1) {
        for j in (i + 1)..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }

    true
}

pub fn ex6_2() -> i32 {
    let mut file = std::fs::File::open("input6.txt").unwrap();
    let mut last_14_chars: VecDeque<u8> = VecDeque::new();
    let mut chars_read = 0;

    loop {
        let mut byte = [0 as u8; 1];
        let bytes_read = file.read(&mut byte).unwrap();

        if bytes_read == 0 {
            unreachable!();
        }

        chars_read += 1;
        last_14_chars.push_back(byte[0]);

        if last_14_chars.len() == 15 {
            last_14_chars.pop_front();
        }

        if last_14_chars.len() == 14 && unique(&last_14_chars) {
            return chars_read;
        }
    }
}
