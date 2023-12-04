use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayFour {}

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut sum = 0;
        for card in contents {
            let card = card.replace("  ", " ");
            let parts = card.split(": ").collect::<Vec<&str>>();
            let content = parts[1];
            let parts = content.split(" | ").collect::<Vec<&str>>();
            let winning_numbers = parts[0].split(" ").map(|v|
                v.parse::<u32>().unwrap()
            ).collect::<Vec<u32>>();
            let numbers = parts[1].split(" ").map(|v|
                v.parse::<u32>().unwrap()
            ).collect::<Vec<u32>>();
            let mut counter = 0;
            for number in numbers {
                if winning_numbers.contains(&number) {
                    counter += 1;
                }
            }
            if counter > 0 {
                sum += 2_u32.pow(counter - 1);
            }
        }
        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut num_copies = vec![1; contents.len()];
        for (i, card) in contents.iter().enumerate() {
            let card = card.replace("  ", " ");
            let parts = card.split(": ").collect::<Vec<&str>>();
            let content = parts[1];
            let parts = content.split(" | ").collect::<Vec<&str>>();
            let winning_numbers = parts[0].split(" ").map(|v|
                v.parse::<u32>().unwrap()
            ).collect::<Vec<u32>>();
            let numbers = parts[1].split(" ").map(|v|
                v.parse::<u32>().unwrap()
            ).collect::<Vec<u32>>();
            let mut counter = 0;
            for number in numbers {
                if winning_numbers.contains(&number) {
                    counter += 1;
                }
            }
            for _ in 0..num_copies[i] {
                for j in (i+1)..(i + counter + 1) {
                    if j < num_copies.len() {
                        num_copies[j] += 1;
                    }
                }
            }
        }
        format!("{}", num_copies.iter().sum::<u32>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}