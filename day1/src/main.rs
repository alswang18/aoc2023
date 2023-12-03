use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    // read input.txt file
    let mut sum: u128 = 0;
    let filename = "./input.txt";
    let file = File::open(filename).expect("Failed to open file");

    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                sum += parse_line_part1(&line) as u128;
            }
            Err(error) => {
                println!("Error reading line: {:?}", error);
                break;
            }
        }
    }
    println!("Sum: {}", sum);
}

fn part_2() {
    // read input.txt file
    let mut sum: u128 = 0;
    let filename = "./input.txt";
    let file = File::open(filename).expect("Failed to open file");

    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                sum += parse_line_part2(&line) as u128;
            }
            Err(error) => {
                println!("Error reading line: {:?}", error);
                break;
            }
        }
    }
    println!("Sum: {}", sum);
}

fn earliest_occurrence(line: &str, word: &str) -> Option<usize> {
    if word.is_empty() || word.len() > line.len() {
        return None;
    }

    let word_size = word.len();
    for start in 0..=line.len() - word_size {
        let end = start + word_size;
        let window = &line[start..end];
        if window == word {
            return Some(start);
        }
    }

    None
}

fn latest_occurrence(line: &str, word: &str) -> Option<usize> {
    if word.is_empty() || word.len() > line.len() {
        return None;
    }

    let word_size = word.len();
    let mut latest = None;
    for start in 0..=line.len() - word_size {
        let end = start + word_size;
        let window = &line[start..end];
        if window == word {
            latest = Some(start);
        }
    }
    latest
}

fn parse_line_part1(line: &str) -> u32 {
    let mut smallest_tuple = (0, usize::MAX);
    let mut largest_tuple = (0, usize::MIN);

    for (i, c) in line.chars().into_iter().enumerate() {
        if c.is_digit(10) {
            if smallest_tuple.1 >= i {
                smallest_tuple = (c.to_digit(10).unwrap() as usize, i);
            }

            if largest_tuple.1 <= i {
                largest_tuple = (c.to_digit(10).unwrap() as usize, i);
            }
        }
    }

    return smallest_tuple.0 as u32 * 10 + largest_tuple.0 as u32;
}

fn parse_line_part2(line: &str) -> u32 {
    let valid_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    // spellings
    let mut smallest_tuple = (0, usize::MAX);
    let mut largest_tuple = (0, usize::MIN);

    for (number, valid_word) in valid_words.iter().enumerate() {
        match earliest_occurrence(line, valid_word) {
            Some(index) => {
                if smallest_tuple.1 >= index {
                    smallest_tuple = (number + 1, index);
                }
                if largest_tuple.1 <= index {
                    largest_tuple = (number + 1, index);
                }
            }
            None => {}
        }
        match latest_occurrence(line, valid_word) {
            Some(index) => {
                if smallest_tuple.1 >= index {
                    smallest_tuple = (number + 1, index);
                }
                if largest_tuple.1 <= index {
                    largest_tuple = (number + 1, index);
                }
            }
            None => {}
        }
    }

    for (i, c) in line.chars().into_iter().enumerate() {
        if c.is_digit(10) {
            if smallest_tuple.1 >= i {
                smallest_tuple = (c.to_digit(10).unwrap() as usize, i);
            }

            if largest_tuple.1 <= i {
                largest_tuple = (c.to_digit(10).unwrap() as usize, i);
            }
        }
    }
    return smallest_tuple.0 as u32 * 10 + largest_tuple.0 as u32;
}
