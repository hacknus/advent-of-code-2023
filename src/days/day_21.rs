use crate::io::read_file_lines;
use crate::problem::Problem;
use std::collections::HashSet;
use std::ops::Rem;

pub struct DayTwentyOne {}

impl Problem for DayTwentyOne {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec![0; contents[0].len()]; contents.len()];
        let mut filled = HashSet::new();
        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                if c == '#' {
                    map[y][x] = 1;
                } else if c == 'S' {
                    map[y][x] = 0;
                    filled.insert((x, y));
                } else {
                    map[y][x] = 0;
                }
            }
        }

        let mut positions_to_append = HashSet::new();

        for step in 0..64 {
            let last_step = filled.clone();
            positions_to_append = HashSet::new();
            for position in filled.iter() {
                // north
                if position.1 > 0 {
                    let mut new_position = position.clone();
                    new_position.1 -= 1;
                    if map[new_position.1][new_position.0] == 0
                        && !last_step.contains(&new_position)
                    {
                        positions_to_append.insert(new_position);
                    }
                }
                // south
                if position.1 < map.len() - 1 {
                    let mut new_position = position.clone();
                    new_position.1 += 1;
                    if map[new_position.1][new_position.0] == 0
                        && !last_step.contains(&new_position)
                    {
                        positions_to_append.insert(new_position);
                    }
                }
                // east
                if position.0 < map[0].len() - 1 {
                    let mut new_position = position.clone();
                    new_position.0 += 1;
                    if map[new_position.1][new_position.0] == 0
                        && !last_step.contains(&new_position)
                    {
                        positions_to_append.insert(new_position);
                    }
                }
                // west
                if position.0 > 0 {
                    let mut new_position = position.clone();
                    new_position.0 -= 1;
                    if map[new_position.1][new_position.0] == 0
                        && !last_step.contains(&new_position)
                    {
                        positions_to_append.insert(new_position);
                    }
                }
            }
            filled = positions_to_append;
        }

        format!("{}", filled.len())
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec![0; contents[0].len()]; contents.len()];
        let mut filled = HashSet::new();
        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                if c == '#' {
                    map[y][x] = 1;
                } else if c == 'S' {
                    map[y][x] = 0;
                    filled.insert((x as isize, y as isize));
                } else {
                    map[y][x] = 0;
                }
            }
        }

        let mut positions_to_append = HashSet::new();

        for step in 0..500 {
            let last_step = filled.clone();
            positions_to_append = HashSet::new();
            for position in filled.iter() {
                // north
                let mut new_position = position.clone();
                new_position.1 -= 1;

                if map[new_position.1.rem_euclid(map.len() as isize) as usize]
                    [new_position.0.rem_euclid(map[0].len() as isize) as usize]
                    == 0
                    && !last_step.contains(&new_position)
                {
                    positions_to_append.insert(new_position);
                }

                // south
                let mut new_position = position.clone();
                new_position.1 += 1;
                if map[new_position.1.rem_euclid(map.len() as isize) as usize]
                    [new_position.0.rem_euclid(map[0].len() as isize) as usize]
                    == 0
                    && !last_step.contains(&new_position)
                {
                    positions_to_append.insert(new_position);
                }

                // east
                let mut new_position = position.clone();
                new_position.0 += 1;
                if map[new_position.1.rem_euclid(map.len() as isize) as usize]
                    [new_position.0.rem_euclid(map[0].len() as isize) as usize]
                    == 0
                    && !last_step.contains(&new_position)
                {
                    positions_to_append.insert(new_position);
                }

                // west
                let mut new_position = position.clone();
                new_position.0 -= 1;
                if map[new_position.1.rem_euclid(map.len() as isize) as usize]
                    [new_position.0.rem_euclid(map[0].len() as isize) as usize]
                    == 0
                    && !last_step.contains(&new_position)
                {
                    positions_to_append.insert(new_position);
                }
            }
            filled = positions_to_append;
        }

        for (y, line) in map.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if filled.contains(&(x as isize, y as isize)) {
                    print!("0");
                } else {
                    if *field == 1 {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
            println!("");
        }

        format!("{}", filled.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
