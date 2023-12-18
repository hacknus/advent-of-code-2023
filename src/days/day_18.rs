use crate::io::read_file_lines;
use crate::problem::Problem;
use colored::{Colorize, CustomColor};
use itertools::min;
use std::collections::HashMap;

pub struct DayEighteen {}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "R" => Self::Right,
            "L" => Self::Left,
            "D" => Self::Down,
            _ => Self::Up,
        }
    }

    fn from_char(s: &char) -> Self {
        match s {
            '0' => Self::Right,
            '2' => Self::Left,
            '1' => Self::Down,
            _ => Self::Up,
        }
    }

    fn as_coordinates(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}
impl Problem for DayEighteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut position_2 = (0, 0);
        let mut position = (0, 0);
        let mut num_positions = 1;
        let mut positions_2 = vec![position_2];

        let mut positions_to_draw = vec![];

        for line in contents.iter() {
            let parts = line
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let direction = Direction::from_str(&parts[0]);
            let amount = parts[1].parse::<isize>().unwrap();
            let color = parts[2].replace("(", "").replace(")", "");
            let color = color.chars().skip(1).take(6).collect::<String>();
            let color = u32::from_str_radix(&color, 16).unwrap();
            num_positions += amount;
            for _ in 0..amount {
                position.0 += direction.as_coordinates().0;
                position.1 += direction.as_coordinates().1;
                positions_to_draw.push((position, color.clone()));
            }
            position_2.0 += direction.as_coordinates().0 * amount;
            position_2.1 += direction.as_coordinates().1 * amount;
            positions_2.push(position_2);
        }

        let min_x = positions_to_draw.iter().map(|(p, _)| p.0).min().unwrap();
        let max_x = positions_to_draw.iter().map(|(p, _)| p.0).max().unwrap();
        let min_y = positions_to_draw.iter().map(|(p, _)| p.1).min().unwrap();
        let max_y = positions_to_draw.iter().map(|(p, _)| p.1).max().unwrap();

        let mut plot_hash = HashMap::new();
        for (p, c) in positions_to_draw {
            let new_p = (p.0 - min_x, p.1 - min_y);
            plot_hash.insert(new_p, c);
        }
        for y in 0..(max_y - min_y) {
            for x in 0..(max_x - min_x) {
                if let Some(c) = plot_hash.get(&(x, y)) {
                    let red = ((c >> 16) & 0xFF) as u8;
                    let green = ((c >> 8) & 0xFF) as u8;
                    let blue = (c & 0xFF) as u8;
                    print!("{}", "#".custom_color(CustomColor::new(red, green, blue)));
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        let mut area = 0;

        for i in 0..(positions_2.len() - 1) {
            area += (positions_2[i + 1].0 + positions_2[i].0)
                * (positions_2[i + 1].1 - positions_2[i].1);
        }

        let straight_pieces = num_positions as usize - positions_2.len();
        let corners = positions_2.len() - 1;

        area /= 2;

        area = area.abs();

        area += straight_pieces as isize / 2;

        let corners = (corners - 4) / 2 + 3;

        area += corners as isize;

        format!("{}", area.abs())
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut position_2 = (0, 0);
        let mut num_positions = 1;
        let mut positions_2 = vec![position_2];
        for line in contents.iter() {
            let parts = line
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let _direction = Direction::from_str(&parts[0]);
            let _amount = parts[1].parse::<isize>().unwrap();
            let color = parts[2].replace("(", "").replace(")", "");
            let direction = Direction::from_char(&color.chars().last().unwrap());
            let amount = color.chars().skip(1).take(5).collect::<String>();
            let amount = isize::from_str_radix(&amount, 16).unwrap();
            num_positions += amount;
            position_2.0 += direction.as_coordinates().0 * amount;
            position_2.1 += direction.as_coordinates().1 * amount;
            positions_2.push(position_2);
        }
        let mut area = 0;

        for i in 0..(positions_2.len() - 1) {
            area += (positions_2[i + 1].0 + positions_2[i].0)
                * (positions_2[i + 1].1 - positions_2[i].1);
        }

        let straight_pieces = num_positions as usize - positions_2.len();
        let corners = positions_2.len() - 1;

        area /= 2;

        area = area.abs();

        area += straight_pieces as isize / 2;

        let corners = (corners - 4) / 2 + 3;

        area += corners as isize;

        format!("{}", area.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
