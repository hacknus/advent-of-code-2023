use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DaySix {}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let times = contents[0].split(":").collect::<Vec<&str>>()[1];
        let times = times.split(" ").collect::<Vec<&str>>();
        let times = times.iter().filter_map(|a| a.parse::<u32>().ok()).collect::<Vec<u32>>();

        let distances = contents[1].split(":").collect::<Vec<&str>>()[1];
        let distances = distances.split(" ").collect::<Vec<&str>>();
        let distances = distances.iter().filter_map(|a| a.parse::<u32>().ok()).collect::<Vec<u32>>();

        let mut product = 1;

        for (time, distance) in times.iter().zip(distances) {
            let mut variants = 0;
            for t in 0..*time {
                let x = t * (*time - t);
                if x > distance {
                    variants += 1;
                }
            }
            product *= variants;
            variants = 0;
        }

        format!("{}", product)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let times = contents[0].split(":").collect::<Vec<&str>>()[1];
        let times = times.replace(" ", "");
        let time = times.parse::<u64>().unwrap();

        let distances = contents[1].split(":").collect::<Vec<&str>>()[1];
        let distances = distances.replace(" ", "");
        let distance = distances.parse::<u64>().unwrap();

        let mut product = 1;

        let mut variants = 0;
        for t in 0..time {
            let x = t * (time - t);
            if x > distance {
                variants += 1;
            }
        }
        product *= variants;
        variants = 0;


        format!("{}", product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}