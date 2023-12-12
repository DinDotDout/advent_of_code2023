use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() -> io::Result<()> {
    let now = Instant::now();
    {
        let path = Path::new("resources/input");
        let file = File::open(&path).expect("Unable to open file");
        let reader = io::BufReader::new(file);
        addup_first_and_last_digits(reader);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}

fn addup_first_and_last_digits(reader: io::BufReader<File>) {
    let digits_regex = Regex::new(r"(\d)").unwrap();
    let mut acumulator = 0;
    for line in reader.lines() {
        let line = line.expect("Error reading line!");
        let mut digits = digits_regex
            .captures_iter(&line)
            .map(|cap| cap[1].to_string());

        let first_digit = digits.next();
        let last_digit = digits.last().or(first_digit.clone());

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            // println!("First digit: {}, Last digit: {}", first, last);
            let number: String = first + &last;
            let my_int: i32 = number.parse().unwrap();
            acumulator += my_int;
        }
    }
    println!("{}", acumulator);
}

