use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
const FILE_PATH: &str = "resources/input";

fn main() -> io::Result<()> {
    let now = Instant::now();
    {
        let path = Path::new(FILE_PATH);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let regex_decimals = Regex::new(r"(\d+)").expect("Invalid regex");

        let mut total_score: u32 = 0;
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            total_score += find_matches_score_hashmap(&parts, &regex_decimals)
            // total_score += find_matches_score_vec(&parts, &regex_decimals)
        }
        println!("Score : {:?}", total_score);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

fn find_matches_score_hashmap(parts: &Vec<&str>, regex_decimals: &Regex) -> u32 {
    let parse_numbers = |s: &str| -> Vec<u32> {
        regex_decimals
            .find_iter(s)
            .filter_map(|m| m.as_str().parse().ok())
            .collect()
    };
    let (left, right): (HashSet<u32>, HashSet<u32>) = (
        parse_numbers(parts[0]).into_iter().skip(1).collect(),
        parse_numbers(parts[1]).into_iter().collect(),
    );

    let intersection: HashSet<_> = left.intersection(&right).collect();
    let matches: usize = intersection.len();
    let score: u32 = if matches == 0 { 0 } else { 1 << matches - 1 };
    score
}

fn find_matches_score_vec(parts: &Vec<&str>, regex_decimals: &Regex) -> u32 {
    let parse_numbers = |s: &str| -> Vec<u32> {
        regex_decimals
            .find_iter(s)
            .filter_map(|m| m.as_str().parse().ok())
            .collect()
    };

    let (mut left, mut right): (Vec<_>, Vec<_>) = (
        parse_numbers(parts[0]).into_iter().skip(1).collect(),
        parse_numbers(parts[1]).into_iter().collect(),
    );

    left.sort_unstable();
    right.sort_unstable();

    let mut i = 0;
    let mut j = 0;
    let mut matches = 0;

    while i < left.len() && j < right.len() {
        match left[i].cmp(&right[j]) {
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
            Ordering::Equal => {
                matches += 1;
                i += 1;
                j += 1;
            }
        }
    }

    let score: u32 = if matches == 0 { 0 } else { 1 << matches - 1 };
    score
}
