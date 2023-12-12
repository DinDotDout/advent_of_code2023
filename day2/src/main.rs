use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

const FILE_PATH: &str = "resources/input";
fn main() -> io::Result<()> {
    let now = Instant::now();
    {
        let path = Path::new(FILE_PATH);
        let file = File::open(&path).expect("Unable to open file");
        let reader = io::BufReader::new(file);

        // Part1
        // sum_valid_games(reader).expect("Failed to sum");

        // Part2
        min_score_valid_games(reader).expect("Failed to get min");
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}

fn min_score_valid_games(reader: io::BufReader<File>) -> io::Result<()> {
    let re_game_results = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut current_game = 1;
    let mut total_power: i32 = 0;
    for line in reader.lines() {
        let line = line?;
        let game_line = re_game_results.captures_iter(&line);
        println!("Game {}", current_game);
        let min_cubes_amount: i32 = min_possible_cubes(game_line);
        total_power += min_cubes_amount;
        current_game += 1;
        println!("");
    }
    println!("Min {}", total_power);
    Ok(())
}

fn min_possible_cubes(line: regex::CaptureMatches) -> i32 {
    let mut max_greens: i32 = 0;
    let mut max_reds: i32 = 0;
    let mut max_blues: i32 = 0;

    for caps in line {
        let number: i32 = caps[1].parse().unwrap();
        let color = &caps[2];

        match color {
            "green" => {
                max_greens = std::cmp::max(max_greens, number);
            }
            "red" => {
                max_reds = std::cmp::max(max_reds, number);
            }
            "blue" => {
                max_blues = std::cmp::max(max_blues, number);
            }
            _ => (),
        }
    }
    return max_greens * max_reds * max_blues;
}

fn sum_valid_games(reader: io::BufReader<File>) -> io::Result<()> {
    let re_game_results = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut score_acumulator = 0;
    let mut current_game = 1;
    for line in reader.lines() {
        let line = line?;
        let game_line = re_game_results.captures_iter(&line);
        println!("Game {}", current_game);
        if is_valid_game(game_line) {
            score_acumulator += current_game;
            println!("current score {}", score_acumulator);
        }
        current_game += 1;
        println!("");
    }
    println!("{}", score_acumulator);
    Ok(())
}

fn is_valid_game(line: regex::CaptureMatches) -> bool {
    const REDS: i32 = 12;
    const GREENS: i32 = 13;
    const BLUES: i32 = 14;
    for caps in line {
        let number: i32 = caps[1].parse().unwrap();
        let color = &caps[2];

        match color {
            "green" => {
                if number > GREENS {
                    println!("greens {}", number);
                    return false;
                }
            }
            "red" => {
                if number > REDS {
                    println!("reds {}", number);
                    return false;
                }
            }
            "blue" => {
                if number > BLUES {
                    println!("blues {}", number);
                    return false;
                }
            }
            _ => (),
        }
    }

    true
}
