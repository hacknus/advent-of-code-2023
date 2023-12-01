use std::num::ParseIntError;
use crate::io::{read_file_lines, read_from_csv};
use crate::problem::Problem;

pub struct DayOne {}

/// this is nasty...
fn get_spelled_digit(c: &str) -> Option<u32> {
    if c.contains("one") {
        Some(1)
    } else if c.contains("two"){
        Some(2)
    } else if c.contains("three") {
        Some(3)
    } else if c.contains("four") {
        Some(4)
    } else if c.contains("five") {
        Some(5)
    } else if c.contains("six") {
        Some(6)
    } else if c.contains("seven") {
        Some(7)
    } else if c.contains("eight") {
        Some(8)
    } else if c.contains("nine") {
        Some(9)
    } else {
        None
    }
}

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut sum = 0;
        for row in contents {
            // get first
            let mut first = 0;
            for char in row.chars() {
                if let Some(digit) = char.to_digit(10) {
                    first = digit;
                    break;
                }
            }

            // get last
            let mut last = 0;
            for char in row.chars().rev() {
                if let Some(digit) = char.to_digit(10) {
                    last = digit;
                    break;
                }
            }
            let val = format!("{}{}", first, last);
            sum += val.parse::<u32>().unwrap();
        }
        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut sum = 0;
        for row in contents {

            // get first
            let mut first = 0;
            let mut slice = "".to_string();
            for char in row.chars() {
                slice += &char.to_string();
                if let Some(digit) = char.to_digit(10) {
                    first = digit;
                    break;
                }
                if let Some(digit) = get_spelled_digit( &slice) {
                    first = digit;
                    break;
                }
            }

            // get last
            let mut last = 0;
            let mut slice = "".to_string();
            for char in row.chars().rev() {
                slice += &char.to_string();
                if let Some(digit) = char.to_digit(10) {
                    last = digit;
                    break;
                }
                if let Some(digit) = get_spelled_digit( &slice.chars().rev().collect::<String>()) {
                    last = digit;
                    break;
                }
            }
            let val = format!("{}{}", first, last);
            sum += val.parse::<u32>().unwrap();
        }
        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}