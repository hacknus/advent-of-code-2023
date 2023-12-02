use crate::io::{read_file_lines, read_from_csv};
use crate::problem::Problem;

pub struct DayTwo {}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut sum = 0;

        for line in contents.iter() {
            let parts = line.split(":").collect::<Vec<&str>>();
            let game_id = parts[0].replace("Game ", "").parse::<u32>().unwrap();

            let game = parts[1];

            let sets = game.split(";").collect::<Vec<&str>>();

            let mut is_valid_set = true;
            for set in sets {
                let mut num_red_cubes = 0;
                let mut num_green_cubes = 0;
                let mut num_blue_cubes = 0;

                let cubes = set.split(",").collect::<Vec<&str>>();
                for cube_num in cubes {
                    if cube_num.contains("red") {
                        let t: String = cube_num.chars().filter(|c| c.is_digit(10)).collect();
                        num_red_cubes += t.parse::<u32>().unwrap();
                    }
                    if cube_num.contains("green") {
                        let t: String = cube_num.chars().filter(|c| c.is_digit(10)).collect();
                        num_green_cubes += t.parse::<u32>().unwrap();
                    }
                    if cube_num.contains("blue") {
                        let t: String = cube_num.chars().filter(|c| c.is_digit(10)).collect();
                        num_blue_cubes += t.parse::<u32>().unwrap();
                    }
                }

                if !(num_red_cubes <= 12 && num_green_cubes <= 13 && num_blue_cubes <= 14) {
                    is_valid_set = false;
                }
            }
            if is_valid_set {
                sum += game_id;
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut sum = 0;

        for line in contents.iter() {
            let parts = line.split(":").collect::<Vec<&str>>();

            let game = parts[1];

            let sets = game.split(";").collect::<Vec<&str>>();

            let mut minimum_red_cubes = 0;
            let mut minimum_green_cubes = 0;
            let mut minimum_blue_cubes = 0;
            for set in sets {
                let mut num_red_cubes = 0;
                let mut num_green_cubes = 0;
                let mut num_blue_cubes = 0;

                let cubes = set.split(",").collect::<Vec<&str>>();
                for cube_num in cubes {
                    if cube_num.contains("red") {
                        let t: String = cube_num.chars().filter(|c| c.is_digit(10)).collect();
                        num_red_cubes += t.parse::<u32>().unwrap();
                    }
                    if cube_num.contains("green") {
                        let t: String = cube_num.chars().filter(|c| c.is_digit(10)).collect();
                        num_green_cubes += t.parse::<u32>().unwrap();
                    }
                    if cube_num.contains("blue") {
                        let t: String = cube_num.chars().filter(|c| c.is_digit(10)).collect();
                        num_blue_cubes += t.parse::<u32>().unwrap();
                    }
                }

                minimum_red_cubes = minimum_red_cubes.max(num_red_cubes);
                minimum_green_cubes = minimum_green_cubes.max(num_green_cubes);
                minimum_blue_cubes = minimum_blue_cubes.max(num_blue_cubes);
            }
            sum += minimum_red_cubes * minimum_green_cubes * minimum_blue_cubes;
        }

        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}