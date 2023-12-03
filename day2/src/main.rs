use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::max;

fn main() {
    part_1();
    part_2();
}

fn is_valid_color(color: &str, value: i32, limits: [(&str, i32); 3]) -> bool {
    let mut valid = false;
    for (name, limit) in limits.iter() {
        if color == *name && value <= *limit {
            valid = true;
            break;
        }
    }
    valid
}

fn part_1(){
    let filename = "./input.txt";
    let file = File::open(filename).expect("Failed to open file");

    let reader = io::BufReader::new(file);
    let limits = [("red", 12), ("green", 13), ("blue", 14)];
    let mut sum_valid_indexes = 0;
    let mut index = 1;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split(":").collect();
                let game = parts[1].trim();
                let mut required: [(&str, i32); 3] = [("red", 0), ("green", 0), ("blue", 0)];
                let rounds: Vec<&str> = game.split(";").collect();
                let mut is_valid: bool = true;
                for round in rounds{
                    let colors: Vec<&str> = round.split(",").collect();
                    for color in colors {
                        let color = color.trim();
                        
                        let color_parts: Vec<&str> = color.split(" ").collect();

                        let color_value = color_parts[0].parse::<i32>().unwrap();
                        let color_name = color_parts[1];
                        required = update_required(color_name, color_value, required);
                        if !is_valid_color(color_name, color_value, limits) {
                            is_valid = false;
                            break;
                        }
                        
                    }
                    if !is_valid {
                        break;
                    }
                }
                let mut power = 1;
                for (_, req) in required.iter() {
                    power *= req;
                }
                if is_valid {
                    sum_valid_indexes += index;
                }
            }
            Err(e) => println!("Error: {}", e),
        }
        index+=1;
    }
    println!("Sum of valid indexes: {}", sum_valid_indexes);
}

fn part_2(){
    let filename = "./input.txt";
    let file = File::open(filename).expect("Failed to open file");

    let reader = io::BufReader::new(file);
    let mut sum_of_powers = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split(":").collect();
                let game = parts[1].trim();
                let mut required: [(&str, i32); 3] = [("red", 0), ("green", 0), ("blue", 0)];
                let rounds: Vec<&str> = game.split(";").collect();
                for round in rounds{
                    let colors: Vec<&str> = round.split(",").collect();
                    for color in colors {
                        let color = color.trim();
                        
                        let color_parts: Vec<&str> = color.split(" ").collect();

                        let color_value = color_parts[0].parse::<i32>().unwrap();
                        let color_name = color_parts[1];
                        required = update_required(color_name, color_value, required);
                    }
                }
                let mut power = 1;
                for (_, req) in required.iter() {
                    power *= req;
                }
                sum_of_powers += power;
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    println!("Sum of powers: {}", sum_of_powers);
}

fn update_required<'a>(color:&'a str, value: i32, required: [(&'a str, i32); 3]) -> [(&'a str, i32); 3] {
    let mut new_required = required;
    for (name, limit) in new_required.iter_mut() {
        
        if color.trim() == *name {
            *limit = max(value, *limit);
            break;
        }
    }
    new_required
}
