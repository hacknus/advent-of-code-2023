use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayFourteen {}

impl Problem for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        dbg!(&contents);
        format!("{}", "Part one not yet implemented.")
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        dbg!(&contents);
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}