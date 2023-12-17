use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct DaySeventeen {}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    dir: Direction,
    position: (usize, usize),
    terminated: bool,
    heat_loss: u32,
    consecutive_moves: usize,
    next_moves: Vec<Rc<RefCell<Move>>>,
}

impl Move {
    pub fn propagate(&mut self, map: &Vec<Vec<String>>) {
        self.heat_loss = map[self.position.1][self.position.0]
            .parse::<u32>()
            .unwrap();
        if self.position == (map[0].len(), map.len()) {
            // we are at the destination
            self.terminated = true;
            return;
        }
        match self.dir {
            Direction::Up => {}
            Direction::Down => {}
            Direction::Left => {}
            Direction::Right => {}
        }
    }
}

impl Problem for DaySeventeen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        format!("{}", "Part one not yet implemented.")
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
