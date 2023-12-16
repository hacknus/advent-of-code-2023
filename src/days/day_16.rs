use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

pub struct DaySixteen {}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Beam {
    dir: Direction,
    position: (usize, usize),
    terminated: bool,
    length: usize,
    children: Vec<Rc<RefCell<Beam>>>,
}

impl Beam {
    pub fn propagate(
        &mut self,
        map: &Vec<Vec<String>>,
        energized_map: &mut Vec<Vec<u64>>,
        states: &mut HashMap<(usize, usize), Direction>,
    ) {
        if let Some(dir) = states.get(&self.position) {
            if self.dir == *dir {
                // already been here
                if energized_map[self.position.1][self.position.0] == 1 {
                    self.terminated = true;
                    return;
                }
            }
        }

        energized_map[self.position.1][self.position.0] = 1;

        states.insert(self.position, self.dir);

        let mut next_position = self.position;

        match self.dir {
            Direction::Up => {
                if next_position.1 > 0 {
                    next_position.1 -= 1;
                } else {
                    self.terminated = true;
                    return;
                }
            }
            Direction::Down => {
                if next_position.1 < map.len() - 1 {
                    next_position.1 += 1;
                } else {
                    self.terminated = true;
                    return;
                }
            }
            Direction::Left => {
                if next_position.0 > 0 {
                    next_position.0 -= 1;
                } else {
                    self.terminated = true;
                    return;
                }
            }
            Direction::Right => {
                if next_position.0 < map[0].len() - 1 {
                    next_position.0 += 1;
                } else {
                    self.terminated = true;
                    return;
                }
            }
        }

        if map[next_position.1][next_position.0] == "|".to_string() {
            match self.dir {
                Direction::Left | Direction::Right => {
                    self.children.push(Rc::new(RefCell::new(Beam {
                        dir: Direction::Up,
                        position: next_position,
                        terminated: false,
                        length: self.length + 1,
                        children: vec![],
                    })));
                    self.children.push(Rc::new(RefCell::new(Beam {
                        dir: Direction::Down,
                        position: next_position,
                        terminated: false,
                        length: self.length + 1,
                        children: vec![],
                    })))
                }
                _ => self.children.push(Rc::new(RefCell::new(Beam {
                    dir: self.dir,
                    position: next_position,
                    terminated: false,
                    length: self.length + 1,
                    children: vec![],
                }))),
            }
        } else if map[next_position.1][next_position.0] == "-".to_string() {
            match self.dir {
                Direction::Up | Direction::Down => {
                    self.children.push(Rc::new(RefCell::new(Beam {
                        dir: Direction::Left,
                        position: next_position,
                        terminated: false,
                        length: self.length + 1,
                        children: vec![],
                    })));
                    self.children.push(Rc::new(RefCell::new(Beam {
                        dir: Direction::Right,
                        position: next_position,
                        terminated: false,
                        length: self.length + 1,
                        children: vec![],
                    })))
                }
                _ => self.children.push(Rc::new(RefCell::new(Beam {
                    dir: self.dir,
                    position: next_position,
                    terminated: false,
                    length: self.length + 1,
                    children: vec![],
                }))),
            }
        } else if map[next_position.1][next_position.0] == "/".to_string() {
            match self.dir {
                Direction::Up => self.children.push(Rc::new(RefCell::new(Beam {
                    dir: Direction::Right,
                    position: next_position,
                    terminated: false,
                    length: self.length + 1,
                    children: vec![],
                }))),
                Direction::Down => self.children.push(Rc::new(RefCell::new(Beam {
                    dir: Direction::Left,
                    position: next_position,
                    terminated: false,
                    length: self.length + 1,
                    children: vec![],
                }))),
                Direction::Left => self.children.push(Rc::new(RefCell::new(Beam {
                    dir: Direction::Down,
                    position: next_position,
                    terminated: false,
                    length: self.length + 1,
                    children: vec![],
                }))),
                Direction::Right => self.children.push(Rc::new(RefCell::new(Beam {
                    dir: Direction::Up,
                    position: next_position,
                    terminated: false,
                    length: self.length + 1,
                    children: vec![],
                }))),
            }
        } else if map[next_position.1][next_position.0] == r"\".to_string() {
            match self.dir {
                Direction::Up => self.children.push(Rc::new(RefCell::new(Beam {
                    dir: Direction::Left,
                    position: next_position,
                    terminated: false,
                    length: self.length + 1,
                    children: vec![],
                }))),
                Direction::Down => self.children.push(Rc::new(RefCell::new(Beam {
                    dir: Direction::Right,
                    position: next_position,
                    terminated: false,
                    length: self.length + 1,
                    children: vec![],
                }))),
                Direction::Left => self.children.push(Rc::new(RefCell::new(Beam {
                    dir: Direction::Up,
                    position: next_position,
                    terminated: false,
                    length: self.length + 1,
                    children: vec![],
                }))),
                Direction::Right => self.children.push(Rc::new(RefCell::new(Beam {
                    dir: Direction::Down,
                    position: next_position,
                    terminated: false,
                    length: self.length + 1,
                    children: vec![],
                }))),
            }
        } else {
            self.children.push(Rc::new(RefCell::new(Beam {
                dir: self.dir,
                position: next_position,
                terminated: false,
                length: self.length + 1,
                children: vec![],
            })))
        }
        for child in self.children.iter_mut() {
            child.borrow_mut().propagate(map, energized_map, states);
        }
    }
}

impl Problem for DaySixteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec!["".to_string(); contents[0].len()]; contents.len()];
        let mut energized_map = vec![vec![0; contents[0].len()]; contents.len()];
        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                map[y][x] = c.to_string();
            }
        }

        let mut states = HashMap::new();

        let dir = match map[0][0].as_str() {
            "|" | r"\" => Direction::Down,
            _ => Direction::Right,
        };

        let mut root = Beam {
            dir,
            position: (0, 0),
            terminated: false,
            length: 0,
            children: vec![],
        };

        root.propagate(&map, &mut energized_map, &mut states);

        let sum = energized_map
            .iter()
            .map(|v| v.iter().sum::<u64>())
            .sum::<u64>();

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec!["".to_string(); contents[0].len()]; contents.len()];
        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                map[y][x] = c.to_string();
            }
        }

        let mut states = HashMap::new();
        let mut sums = vec![];

        let mut edge_tiles_top = (0..map[0].len())
            .map(|x| (x, 0, Direction::Down))
            .collect::<Vec<(usize, usize, Direction)>>();
        let mut edge_tiles_right = (0..map.len())
            .map(|y| (map[0].len() - 1, y, Direction::Left))
            .collect::<Vec<(usize, usize, Direction)>>();
        let mut edge_tiles_left = (0..map.len())
            .map(|y| (0, y, Direction::Right))
            .collect::<Vec<(usize, usize, Direction)>>();
        let mut edge_tiles_bottom = (0..map[0].len())
            .map(|x| (x, map.len() - 1, Direction::Up))
            .collect::<Vec<(usize, usize, Direction)>>();

        let mut edge_tiles = vec![];
        edge_tiles.append(&mut edge_tiles_top);
        edge_tiles.append(&mut edge_tiles_right);
        edge_tiles.append(&mut edge_tiles_left);
        edge_tiles.append(&mut edge_tiles_bottom);

        for (x, y, dir) in edge_tiles {
            let mut energized_map = vec![vec![0; contents[0].len()]; contents.len()];
            let mut children = vec![];

            let dir = match dir {
                Direction::Up => match map[y][x].as_str() {
                    "-" => {
                        children.push(Rc::new(RefCell::new(Beam {
                            dir: Direction::Left,
                            position: (x, y),
                            terminated: false,
                            length: 0,
                            children: vec![],
                        })));
                        Direction::Right
                    }
                    r"\" => Direction::Left,
                    "/" => Direction::Right,
                    _ => Direction::Up,
                },
                Direction::Down => match map[y][x].as_str() {
                    "-" => {
                        children.push(Rc::new(RefCell::new(Beam {
                            dir: Direction::Left,
                            position: (x, y),
                            terminated: false,
                            length: 0,
                            children: vec![],
                        })));
                        Direction::Right
                    }
                    r"\" => Direction::Right,
                    "/" => Direction::Left,
                    _ => Direction::Down,
                },
                Direction::Left => match map[y][x].as_str() {
                    "|" => {
                        children.push(Rc::new(RefCell::new(Beam {
                            dir: Direction::Up,
                            position: (x, y),
                            terminated: false,
                            length: 0,
                            children: vec![],
                        })));
                        Direction::Down
                    }
                    r"\" => Direction::Up,
                    "/" => Direction::Down,
                    _ => Direction::Left,
                },
                Direction::Right => match map[y][x].as_str() {
                    "|" => {
                        children.push(Rc::new(RefCell::new(Beam {
                            dir: Direction::Up,
                            position: (x, y),
                            terminated: false,
                            length: 0,
                            children: vec![],
                        })));
                        Direction::Down
                    }
                    r"\" => Direction::Down,
                    "/" => Direction::Up,
                    _ => Direction::Right,
                },
            };

            let mut root = Beam {
                dir,
                position: (x, y),
                terminated: false,
                length: 0,
                children,
            };

            root.propagate(&map, &mut energized_map, &mut states);

            let sum = energized_map
                .iter()
                .map(|v| v.iter().sum::<u64>())
                .sum::<u64>();
            sums.push(sum);
        }

        format!("{}", sums.iter().max().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
