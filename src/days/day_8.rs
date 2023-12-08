use num::Integer;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayEight {}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    children: Vec<String>,
}


fn lcm_of_vector(numbers: &[usize]) -> usize {
    if numbers.is_empty() {
        return 0;
    }

    let mut result = numbers[0];
    for num in &numbers[1..] {
        result = result.lcm(num);
    }
    result
}

impl Problem for DayEight {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let moves = contents[0].clone();
        let nodes_string = &contents[2..];
        let mut nodes = vec![];
        for node in nodes_string {
            let parts = node.to_string().split(" = ").map(|s| s.to_string()).collect::<Vec<String>>();
            let name = parts[0].clone();
            let children = parts[1].replace("(", "").replace(")", "").split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
            nodes.push(Node { name, children })
        }
        let mut position = "AAA";
        let mut move_ind = 0;
        let mut move_counter = 0;
        loop {
            // start with AAA
            let current_move = if moves.chars().nth(move_ind).unwrap() == 'R' {
                1
            } else {
                0
            };
            for node in nodes.iter() {
                if node.name == position {
                    position = &node.children[current_move];
                    break;
                }
            }
            move_ind += 1;
            move_counter += 1;
            if move_ind == moves.len() {
                move_ind = 0;
            }
            if position == "ZZZ" {
                break;
            }
        }
        format!("{}", move_counter)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let moves = contents[0].clone();
        let nodes_string = &contents[2..];
        let mut nodes = vec![];
        for node in nodes_string {
            let parts = node.to_string().split(" = ").map(|s| s.to_string()).collect::<Vec<String>>();
            let name = parts[0].clone();
            let children = parts[1].replace("(", "").replace(")", "").split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
            nodes.push(Node { name, children })
        }
        let mut positions = nodes.iter().filter_map(|node| {
            if node.name.contains("A") {
                Some(node.name.clone())
            } else { None }
        }).collect::<Vec<String>>();

        let mut nums = vec![0; positions.len()];

        for (i, position) in positions.iter_mut().enumerate() {
            let mut move_counter = 0;
            loop {
                for m in moves.chars() {
                    move_counter += 1;
                    let current_move = if m == 'R' {
                        1
                    } else {
                        0
                    };

                    for node in nodes.iter() {
                        if position.contains(&node.name) {
                            *position = node.children[current_move].clone();
                            break;
                        }
                    }

                    if position.contains("Z") {
                        nums[i] = move_counter;
                        break;
                    }
                }
                if nums[i] != 0 {
                    break;
                }
            }
        }
        let result = lcm_of_vector(&nums);
        format!("{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}