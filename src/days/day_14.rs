use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayFourteen {}

impl Problem for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut round_rocks = vec![];
        let mut cube_shaped_rocks = vec![];
        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                match c {
                    '#' => cube_shaped_rocks.push((x, y)),
                    'O' => round_rocks.push((x, y)),
                    _ => {}
                }
            }
        }

        // roll to north
        for i in 0..round_rocks.len() {
            let (x, mut y) = round_rocks[i];
            loop {
                if y > 0 && !cube_shaped_rocks.contains(&(x, y - 1)) && !round_rocks.contains(&(x, y - 1)) {
                    y -= 1;
                    round_rocks[i] = (x, y);
                } else {
                    break;
                }
            }
        }


        // sum all loads
        let mut summed_load = 0;
        let max_y = contents.len();
        for round_rock in round_rocks {
            summed_load += max_y - round_rock.1
        }

        format!("{}", summed_load)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut round_rocks = vec![];
        let mut cube_shaped_rocks = vec![];
        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                match c {
                    '#' => cube_shaped_rocks.push((x, y)),
                    'O' => round_rocks.push((x, y)),
                    _ => {}
                }
            }
        }

        for _cycle in 0..4 {
            // if _cycle % 10000 == 0 {
            //    println!("cycle: {_cycle}/1000000000");
            // }

            // roll to north
            for i in 0..round_rocks.len() {
                let (x, mut y) = round_rocks[i];
                loop {
                    if y > 0 && !cube_shaped_rocks.contains(&(x, y - 1)) && !round_rocks.contains(&(x, y - 1)) {
                        y -= 1;
                        round_rocks[i] = (x, y);
                    } else {
                        break;
                    }
                }
            }


            // roll to west
            for i in 0..round_rocks.len() {
                let (mut x, y) = round_rocks[i];
                loop {
                    if x > 0 && !cube_shaped_rocks.contains(&(x - 1, y)) && !round_rocks.contains(&(x - 1, y)) {
                        x -= 1;
                        round_rocks[i] = (x, y);
                    } else {
                        break;
                    }
                }
            }

            // roll to east
            for i in (0..round_rocks.len()).rev() {
                let (mut x, y) = round_rocks[i];
                loop {
                    if x < contents[0].len() - 1 && !cube_shaped_rocks.contains(&(x + 1, y)) && !round_rocks.contains(&(x + 1, y)) {
                        x += 1;
                        round_rocks[i] = (x, y);
                    } else {
                        break;
                    }
                }
            }

            // roll to south
            for i in (0..round_rocks.len()).rev() {
                let (x, mut y) = round_rocks[i];
                loop {
                    if y < contents.len() - 1 && !cube_shaped_rocks.contains(&(x, y + 1)) && !round_rocks.contains(&(x, y + 1)) {
                        y += 1;
                        round_rocks[i] = (x, y);
                    } else {
                        break;
                    }
                }
            }
            // sum all loads
            let mut summed_load = 0;
            let max_y = contents.len();
            for round_rock in round_rocks.iter() {
                summed_load += max_y - round_rock.1
            }
            println!("summed loads: {}", summed_load);
        }
        // sum all loads
        let mut summed_load = 0;
        let max_y = contents.len();
        for round_rock in round_rocks.iter() {
            summed_load += max_y - round_rock.1
        }

        format!("{}", summed_load)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}