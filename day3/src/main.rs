use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
const FILE_PATH: &str = "resources/input";

struct LineTracker<R: BufRead> {
    reader: R,
    prev_line: String,
    current_line: String,
    next_line: String,
}

impl<R: BufRead> LineTracker<R> {
    fn new(mut reader: R) -> io::Result<Self> {
        let mut next_line = String::new();
        reader.read_line(&mut next_line)?;
        Ok(Self {
            reader,
            prev_line: "".to_string(),
            current_line: "".to_string(),
            next_line,
        })
    }
}

impl<R: BufRead> Iterator for LineTracker<R> {
    type Item = io::Result<(String, String, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_line.is_empty() {
            return None;
        }
        self.prev_line = self.current_line.clone();
        self.current_line = self.next_line.clone();
        let mut next_line = String::new();

        match self.reader.read_line(&mut next_line) {
            // If there are no more lines to read, return the result
            Ok(0) => {
                self.next_line = "".to_string();
                Some(Ok((
                    self.prev_line.clone(),
                    self.current_line.clone(),
                    "".to_string(),
                )))
            }

            // If a line was read, update next_line and return the result
            Ok(_) => {
                self.next_line = next_line;
                Some(Ok((
                    self.prev_line.clone(),
                    self.current_line.clone(),
                    self.next_line.clone(),
                )))
            }

            Err(e) => Some(Err(e)),
        }
    }
}

fn main() -> io::Result<()> {
    let now = Instant::now();
    {
        let path = Path::new(FILE_PATH);
        let file = File::open(&path).expect("Unable to open file");
        let reader = io::BufReader::new(file);

        // Part1
        engine_matches(reader);

        // Part2
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}

fn engine_matches(reader: io::BufReader<File>) {
    let mut line_tracker = LineTracker::new(reader).expect("Could not initialize LineTracker");
    let mut total_score: u32 = 0;
    while let Some(result) = line_tracker.next() {
        match result {
            Ok(value) => {
                let (prev_line, current_line, next_line) = value;
                // println!("{}", current_line);
                total_score += find_matches(prev_line, current_line, next_line);
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
    println!("{}", total_score);
}

fn find_matches(prev_line: String, current_line: String, next_line: String) -> u32 {
    let re_decimals = Regex::new(r"(\d+)").expect("Invalid regex");
    let re_symbols = Regex::new(r"[^\w.\s]").expect("Invalid regex");

    let line = &current_line.clone();
    let line_length: usize = current_line.len() - 1;
    let mut line_score: u32 = 0;
    for decimal_idxs in re_decimals.find_iter(line) {
        let (start, end) = (decimal_idxs.start(), decimal_idxs.end());
        let number_str = &line[decimal_idxs.start()..decimal_idxs.end()];
        let number: u32 = number_str.parse().expect("Failed to parse number");

        for i in start..end {
            let should_check_left = i == start && i > 0;
            let should_check_right = i == end - 1 && i < line_length;

            if check_middle(&prev_line, &next_line, i, &re_symbols)
                || (should_check_left
                    && check_left(&prev_line, &current_line, &next_line, i, &re_symbols))
                || (should_check_right
                    && check_right(&prev_line, &current_line, &next_line, i, &re_symbols))
            {
                line_score += number;
                break;
            }
        }
    }
    return line_score;
}

fn check_middle(prev_line: &String, next_line: &String, idx: usize, regex: &Regex) -> bool {
    check_match(prev_line, idx, regex) || check_match(next_line, idx, regex)
}

fn check_left(
    prev_line: &String,
    current_line: &String,
    next_line: &String,
    idx: usize,
    regex: &Regex,
) -> bool {
    let idx = idx - 1; // Check the character to the left
    check_match(current_line, idx, regex)
        || check_match(prev_line, idx, regex)
        || check_match(next_line, idx, regex)
}

fn check_right(
    prev_line: &String,
    current_line: &String,
    next_line: &String,
    idx: usize,
    regex: &Regex,
) -> bool {
    let idx = idx + 1; // Check the character to the right
    check_match(current_line, idx, regex)
        || check_match(prev_line, idx, regex)
        || check_match(next_line, idx, regex)
}

fn check_match(line: &String, idx: usize, regex: &Regex) -> bool {
    if line.is_empty() {
        return false;
    }
    if let Some(c) = line.chars().nth(idx) {
        return regex.is_match(&c.to_string());
    }

    false
}
