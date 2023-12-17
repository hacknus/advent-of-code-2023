use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct DaySeventeen {}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_coordinates(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Direction::Up => "^".to_string(),
            Direction::Down => "v".to_string(),
            Direction::Left => "<".to_string(),
            Direction::Right => ">".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    position: (isize, isize),
    directions: Vec<Direction>,
    g: i32,
    h: i32,
    f: i32,
}

fn a_star(
    map: &Vec<Vec<i32>>,
    initial_direction: Direction,
    start: (isize, isize),
    end: (isize, isize),
) -> Option<Vec<((isize, isize), Direction)>> {
    // create start and end node
    let start_node = Node {
        parent: None,
        position: start,
        directions: vec![initial_direction],
        g: 0,
        h: 0,
        f: 0,
    };
    let end_node = Node {
        parent: None,
        position: end,
        directions: vec![],
        g: 0,
        h: 0,
        f: 0,
    };

    // initialize both open and closed list
    let mut open_list = vec![];
    let mut closed_list = vec![];

    open_list.push(start_node);

    // loop until we find the end
    while open_list.len() > 0 {
        let mut current_node = open_list[0].clone();
        let mut current_index = 0;

        for (index, item) in open_list.iter().enumerate() {
            if item.f < current_node.f {
                current_node = item.clone();
                current_index = index
            }
        }

        // Pop current off open list, add to closed list
        open_list.remove(current_index);
        closed_list.push(current_node.clone());

        // Found the end
        if current_node.position == end_node.position {
            let mut path = vec![(
                current_node.position,
                current_node.directions.last().unwrap().clone(),
            )];
            let mut current = current_node.clone();
            while let Some(parent) = current.parent {
                path.push((
                    parent.borrow().position,
                    parent.borrow().directions.last().unwrap().clone(),
                ));
                current = parent.borrow().clone();
            }
            return Some(path);
        }

        // generate children

        let mut children = vec![];
        for new_direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            // Get node position
            let node_position = (
                current_node.position.0 + new_direction.as_coordinates().0,
                current_node.position.1 + new_direction.as_coordinates().1,
            );

            // Make sure within range
            if node_position.0 >= map[0].len() as isize
                || node_position.0 < 0
                || node_position.1 >= map.len() as isize
                || node_position.1 < 0
            {
                continue;
            }

            // Make sure walkable terrain
            // if map[node_position.1 as usize][node_position.0 as usize] != 0 {
            //     continue;
            // }

            let mut directions = current_node.directions.clone();
            directions.push(new_direction);

            // Create new node
            let new_node = Node {
                parent: Some(Rc::new(RefCell::new(current_node.clone()))),
                position: node_position,
                directions,
                g: 0,
                h: 0,
                f: 0,
            };

            // Append
            children.push(new_node);
        }
        // Loop through children
        'outer: for mut child in children.iter_mut() {
            // Child is on the closed list
            for closed_child in closed_list.iter() {
                if child.position == closed_child.position {
                    continue 'outer;
                }
            }

            // logic for 3 moves in a row:
            let n = child.directions.len();
            if n >= 4 {
                if child.directions[n - 1] == child.directions[n - 2]
                    && child.directions[n - 1] == child.directions[n - 3]
                    && child.directions[n - 1] == child.directions[n - 4]
                {
                    continue;
                }
            }

            // Create the f, g, and h values
            child.g =
                current_node.g + 1 + map[child.position.1 as usize][child.position.0 as usize];
            // child.g = current_node.g + 1;
            child.h = ((child.position.0 as i32 - end_node.position.0 as i32).pow(2))
                + ((child.position.1 as i32 - end_node.position.1 as i32).pow(2));
            child.h = 0;
            child.f = child.g + child.h;

            // Child is already in the open list
            for open_node in open_list.iter() {
                if child.position == open_node.position && child.g > open_node.g {
                    continue 'outer;
                }
            }

            // Add the child to the open list
            open_list.push(child.clone());
        }
    }
    None
}

impl Problem for DaySeventeen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec![0; contents[0].len()]; contents.len()];
        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                map[y][x] = c.to_digit(10).unwrap() as i32;
            }
        }

        let start = (1, 0);
        let end = (map[0].len() as isize - 1, map.len() as isize - 1);
        let initial_direction = Direction::Right;

        let path = a_star(&map, initial_direction, start, end).unwrap();
        for (y, line) in map.iter().enumerate() {
            for (x, local_heat_loss) in line.iter().enumerate() {
                let mut b = false;
                for ((x_p, y_p), dir) in path.iter() {
                    if x == *x_p as usize && y == *y_p as usize {
                        b = true;
                        print!("{}", dir.to_string());
                    }
                }
                if !b {
                    print!("{local_heat_loss}");
                }
            }
            println!();
        }
        let heat_loss = path
            .iter()
            .map(|((x, y), _)| map[*y as usize][*x as usize])
            .sum::<i32>();
        format!("{}", heat_loss)
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
