use std::collections::HashSet;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayThree {}

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut sum = 0;
        for i in 0..contents.len() {
            let mut number = "".to_string();
            let mut is_adjacent = false;
            for (j, c) in contents[i].char_indices() {
                if c.is_digit(10) {
                    number += &c.to_string();
                    // check if there is a symbol
                    // left:
                    let left = contents[i].chars().nth(j - 1).unwrap_or("0".parse().unwrap());
                    if !left.is_digit(10) && left != '.' {
                        is_adjacent = true;
                    }
                    // right:
                    let right = contents[i].chars().nth(j + 1).unwrap_or("0".parse().unwrap());
                    if !right.is_digit(10) && right != '.' {
                        is_adjacent = true;
                    }
                    // up:
                    if i > 0 {
                        let up = contents[i - 1].chars().nth(j).unwrap_or("0".parse().unwrap());
                        if !up.is_digit(10) && up != '.' {
                            is_adjacent = true;
                        }
                    }
                    if i < contents.len() - 1 {
                        // down:
                        let down = contents[i + 1].chars().nth(j).unwrap_or("0".parse().unwrap());
                        if !down.is_digit(10) && down != '.' {
                            is_adjacent = true;
                        }
                    }
                    // left up:
                    if i > 0 {
                        let left_up = contents[i - 1].chars().nth(j - 1).unwrap_or("0".parse().unwrap());
                        if !left_up.is_digit(10) && left_up != '.' {
                            is_adjacent = true;
                        }
                    }
                    // left down:
                    if i < contents.len() - 1 {
                        let left_down = contents[i + 1].chars().nth(j - 1).unwrap_or("0".parse().unwrap());
                        if !left_down.is_digit(10) && left_down != '.' {
                            is_adjacent = true;
                        }
                    }
                    // right up:
                    if i > 0 {
                        let right_up = contents[i - 1].chars().nth(j + 1).unwrap_or("0".parse().unwrap());
                        if !right_up.is_digit(10) && right_up != '.' {
                            is_adjacent = true;
                        }
                    }
                    // right down:
                    if i < contents.len() - 1 {
                        let right_down = contents[i + 1].chars().nth(j + 1).unwrap_or("0".parse().unwrap());
                        if !right_down.is_digit(10) && right_down != '.' {
                            is_adjacent = true;
                        }
                    }
                } else {
                    if number != "".to_string() {
                        if is_adjacent {
                            sum += number.parse::<u32>().unwrap();
                        }
                    }
                    is_adjacent = false;
                    number = "".to_string();
                }
                if j == contents[i].len() - 1 {
                    if number != "".to_string() {
                        if is_adjacent {
                            sum += number.parse::<u32>().unwrap();
                        }
                    }
                    is_adjacent = false;
                    number = "".to_string();
                }
            }
        }
        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut sum = 0;
        let mut gears = vec![];
        for i in 0..contents.len() {
            let mut numbers = [0; 8];
            for (j, c) in contents[i].char_indices() {
                if c == '*' {
                    gears.push([i, j]);
                }
            }
        }

        for [i, j] in gears {
            let mut numbers = [0; 8];

            // check if there are numbers close by
            // left:
            if j > 0 {
                let mut number = "".to_string();
                let mut next_j = 1;
                while let Some(left) = contents[i].chars().nth(j - next_j) {
                    if left.is_digit(10) {
                        number += &left.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }
                if !number.is_empty() {
                    numbers[0] = number.chars().rev().collect::<String>().parse::<u32>().unwrap();
                }
            }
            // right:
            if j < contents[i].len() {
                let mut number = "".to_string();
                let mut next_j = 1;
                while let Some(right) = contents[i].chars().nth(j + next_j) {
                    if right.is_digit(10) {
                        number += &right.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }
                if !number.is_empty() {
                    numbers[1] = number.parse::<u32>().unwrap();
                }
            }
            // up:
            if i > 0 {
                let mut number_left = "".to_string();
                let mut next_j = 0;
                while let Some(left) = contents[i-1].chars().nth(j - next_j) {
                    if left.is_digit(10) {
                        number_left += &left.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let mut number_right = "".to_string();
                let mut next_j = 1;
                while let Some(right) = contents[i-1].chars().nth(j + next_j) {
                    if right.is_digit(10) {
                        number_right += &right.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let number = number_left.chars().rev().collect::<String>() + &number_right;
                if !number.is_empty() {
                    numbers[2] = number.parse::<u32>().unwrap();
                }
            }
            // down:
            if i < contents.len() - 1 {
                let mut number_left = "".to_string();
                let mut next_j = 0;
                while let Some(left) = contents[i+1].chars().nth(j - next_j) {
                    if left.is_digit(10) {
                        number_left += &left.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let mut number_right = "".to_string();
                let mut next_j = 1;
                while let Some(right) = contents[i+1].chars().nth(j + next_j) {
                    if right.is_digit(10) {
                        number_right += &right.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let number = number_left.chars().rev().collect::<String>() + &number_right;
                if !number.is_empty() {
                    numbers[3] = number.parse::<u32>().unwrap();
                }
            }
            // left up:
            if i > 0 && j > 1 && numbers[2] == 0 {
                let mut number_left = "".to_string();
                let mut next_j = 1;
                while let Some(left) = contents[i - 1].chars().nth(j - next_j) {
                    if left.is_digit(10) {
                        number_left += &left.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let mut number_right = "".to_string();
                let mut next_j = 0;
                while let Some(right) = contents[i - 1].chars().nth(j + next_j) {
                    if right.is_digit(10) {
                        number_right += &right.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let number = number_left.chars().rev().collect::<String>() + &number_right;
                if !number.is_empty() {
                    numbers[4] = number.parse::<u32>().unwrap();
                }
            }
            // left down :
            if i < contents.len() - 1 && j > 0  {
                let mut number_left = "".to_string();
                let mut next_j = 1;
                while let Some(left) = contents[i + 1].chars().nth(j - next_j) {
                    if left.is_digit(10) {
                        number_left += &left.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let mut number_right = "".to_string();
                let mut next_j = 0;
                while let Some(right) = contents[i + 1].chars().nth(j + next_j) {
                    if right.is_digit(10) {
                        number_right += &right.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let number = number_left.chars().rev().collect::<String>() + &number_right;
                if !number.is_empty() {
                    numbers[5] = number.parse::<u32>().unwrap();
                }
            }
            // right up:
            if i > 0 && j < contents[i].len() {
                let mut number_left = "".to_string();
                let mut next_j = 1;
                while let Some(left) = contents[i - 1].chars().nth(j - next_j) {
                    if left.is_digit(10) {
                        number_left += &left.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let mut number_right = "".to_string();
                let mut next_j = 0;
                while let Some(right) = contents[i - 1].chars().nth(j + next_j) {
                    if right.is_digit(10) {
                        number_right += &right.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let number = number_left.chars().rev().collect::<String>() + &number_right;
                if !number.is_empty() {
                    numbers[6] = number.parse::<u32>().unwrap();
                }
            }
            // right down:
            if i < contents.len() - 1 && j < contents[i].len() {
                let mut number_left = "".to_string();
                let mut next_j = 1;
                while let Some(left) = contents[i + 1].chars().nth(j - next_j) {
                    if left.is_digit(10) {
                        number_left += &left.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let mut number_right = "".to_string();
                let mut next_j = 0;
                while let Some(right) = contents[i + 1].chars().nth(j + next_j) {
                    if right.is_digit(10) {
                        number_right += &right.to_string();
                        next_j += 1;
                    } else {
                        break;
                    }
                }

                let number = number_left.chars().rev().collect::<String>() + &number_right;
                if !number.is_empty() {
                    numbers[7] = number.parse::<u32>().unwrap();
                }
            }
            let non_zero_gears = numbers.iter().filter(|&v| *v != 0).collect::<Vec<&u32>>();
            let unique_gears = non_zero_gears.into_iter()
                .collect::<HashSet<_>>()
                .iter()
                .map(|&v| *v)
                .collect::<Vec<u32>>();
            if unique_gears.len() != 2 {
                continue
            }
            sum += unique_gears.iter().product::<u32>();
        }


        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}