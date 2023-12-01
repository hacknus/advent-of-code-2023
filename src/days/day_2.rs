use crate::io::{read_file_lines, read_from_csv};
use crate::problem::Problem;

pub struct DayTwo {}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        println!("opening {input}");
        let contents = read_file_lines(input);
        println!("found contents {contents:?}");

        format!("{}", "Part one not yet implemented.")
    }

    fn part_two(&self, input: &str) -> String {
        println!("opening {input}");
        let contents = read_from_csv(input, b',');
        println!("found contents {contents:?}");

        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}