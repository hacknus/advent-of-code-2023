use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayTen {}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Problem for DayTen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let map = contents.iter().map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

        let mut positions = vec![];
        let mut pos = (0, 0);
        let mut vector = Direction::Down;

        for (y, line) in map.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if field == "S" {
                    positions.push((x, y));
                    pos.0 = x;
                    pos.1 = y;

                    // lookout north
                    if pos.1 > 0 {
                        let field = &map[pos.1 - 1][pos.0];
                        if ["|", "7", "F"].contains(&field.as_str()) {
                            vector = Direction::Up
                        }
                    }

                    // lookout south
                    if pos.1 < map.len() {
                        let field = &map[pos.1 + 1][pos.0];
                        if ["|", "L", "J"].contains(&field.as_str()) {
                            vector = Direction::Down
                        }
                    }

                    // lookout left
                    if pos.0 > 0 {
                        let field = &map[pos.1][pos.0 - 1];
                        if ["-", "F", "L"].contains(&field.as_str()) {
                            vector = Direction::Left
                        }
                    }

                    // lookout right
                    if pos.0 < map[0].len() {
                        let field = &map[pos.1][pos.0 + 1];
                        if ["-", "J", "7"].contains(&field.as_str()) {
                            vector = Direction::Right
                        }
                    }
                }
            }
        }

        loop {
            match vector {
                Direction::Up => {
                    pos.1 -= 1;
                }
                Direction::Down => {
                    pos.1 += 1;
                }
                Direction::Left => {
                    pos.0 -= 1;
                }
                Direction::Right => {
                    pos.0 += 1;
                }
            }

            positions.push(pos);
            let field = &map[pos.1][pos.0];


            match field.as_str() {
                "|" => {
                    // continue in straight line
                }
                "-" => {
                    // continue in straight line
                }
                "L" => {
                    if vector == Direction::Down {
                        vector = Direction::Right
                    } else if vector == Direction::Left {
                        vector = Direction::Up
                    }
                }
                "J" => {
                    if vector == Direction::Down {
                        vector = Direction::Left
                    } else if vector == Direction::Right {
                        vector = Direction::Up
                    }
                }
                "7" => {
                    if vector == Direction::Up {
                        vector = Direction::Left
                    } else if vector == Direction::Right {
                        vector = Direction::Down
                    }
                }
                "F" => {
                    if vector == Direction::Up {
                        vector = Direction::Right
                    } else if vector == Direction::Left {
                        vector = Direction::Down
                    }
                }
                "S" => {
                    if positions.len() > 1 {
                        break;
                    }
                }
                _ => {
                    panic!("this should not happen")
                }
            }
        }
        format!("{}", positions.len() / 2)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = contents.iter().map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

        // do first part

        let mut positions = vec![];
        let mut pos = (0, 0);
        let mut vector = Direction::Down;

        for (y, line) in map.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if field == "S" {
                    positions.push((x, y));
                    pos.0 = x;
                    pos.1 = y;

                    // lookout north
                    if pos.1 > 0 {
                        let field = &map[pos.1 - 1][pos.0];
                        if ["|", "7", "F"].contains(&field.as_str()) {
                            vector = Direction::Up
                        }
                    }

                    // lookout south
                    if pos.1 < map.len() {
                        let field = &map[pos.1 + 1][pos.0];
                        if ["|", "L", "J"].contains(&field.as_str()) {
                            vector = Direction::Down
                        }
                    }

                    // lookout left
                    if pos.0 > 0 {
                        let field = &map[pos.1][pos.0 - 1];
                        if ["-", "F", "L"].contains(&field.as_str()) {
                            vector = Direction::Left
                        }
                    }

                    // lookout right
                    if pos.0 < map[0].len() {
                        let field = &map[pos.1][pos.0 + 1];
                        if ["-", "J", "7"].contains(&field.as_str()) {
                            vector = Direction::Right
                        }
                    }
                }
            }
        }

        let initial_vector = vector.clone();

        loop {
            match vector {
                Direction::Up => {
                    pos.1 -= 1;
                }
                Direction::Down => {
                    pos.1 += 1;
                }
                Direction::Left => {
                    pos.0 -= 1;
                }
                Direction::Right => {
                    pos.0 += 1;
                }
            }

            let field = &map[pos.1][pos.0];


            match field.as_str() {
                "|" => {
                    // continue in straight line
                }
                "-" => {
                    // continue in straight line
                }
                "L" => {
                    if vector == Direction::Down {
                        vector = Direction::Right
                    } else if vector == Direction::Left {
                        vector = Direction::Up
                    }
                }
                "J" => {
                    if vector == Direction::Down {
                        vector = Direction::Left
                    } else if vector == Direction::Right {
                        vector = Direction::Up
                    }
                }
                "7" => {
                    if vector == Direction::Up {
                        vector = Direction::Left
                    } else if vector == Direction::Right {
                        vector = Direction::Down
                    }
                }
                "F" => {
                    if vector == Direction::Up {
                        vector = Direction::Right
                    } else if vector == Direction::Left {
                        vector = Direction::Down
                    }
                }
                "S" => {
                    if positions.len() > 1 {
                        break;
                    }
                }
                _ => {
                    panic!("this should not happen")
                }
            }
            positions.push(pos);

        }

        for (y, line) in map.iter_mut().enumerate() {
            for (x, field) in line.iter_mut().enumerate() {
                if !positions.contains(&(x, y)) {
                    *field = ".".to_string();
                }
            }
        }

        // replace the start symbol
        let difference = (positions[1].0 as isize - positions.last().unwrap().0 as isize, positions[1].1 as isize - positions.last().unwrap().1 as isize);

        map[positions[0].1][positions[0].0] = match difference {
            (-1, -1) => {"F".to_string()},
            (0, 1) => {"-".to_string()},
            (1, 0) => {"|".to_string()},
            (1, -1) => {"7".to_string()},
            (-1, 1) => {"L".to_string()},
            (1, 1) => {"J".to_string()},
            (0, -1) => {"-".to_string()},
            (-1, 0) => {"|".to_string()},
            (_, _) => {"S".to_string()},
        };




        let mut sum = 0;

        for (y, line) in map.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if field == "." {
                    let look_up = map.iter().map(|line| line[x].clone()).take(y).collect::<Vec<String>>();
                    let look_down = map.iter().map(|line| line[x].clone()).skip(y).collect::<Vec<String>>();
                    let look_left = &line[0..x];
                    let look_right = &line[x..];
                    if look_left.iter().filter(|&v| *v == "|" || *v == "F" || *v == "7").count() % 2 != 0 &&
                        look_left.iter().filter(|&v| *v == "|" || *v == "L" || *v == "J").count() % 2 != 0 &&
                        look_right.iter().filter(|&v| *v == "|" || *v == "F" || *v == "7").count() % 2 != 0 &&
                        look_right.iter().filter(|&v| *v == "|" || *v == "L" || *v == "J").count() % 2 != 0 &&
                        look_up.iter().filter(|&v| *v == "-" || *v == "F" || *v == "L").count() % 2 != 0 &&
                        look_up.iter().filter(|&v| *v == "-" || *v == "7" || *v == "J").count() % 2 != 0 &&
                        look_down.iter().filter(|&v| *v == "-" || *v == "F" || *v == "L").count() % 2 != 0 &&
                        look_down.iter().filter(|&v| *v == "-" || *v == "7" || *v == "J").count() % 2 != 0
                    {
                        sum += 1;
                    }
                }
            }
        }

        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}