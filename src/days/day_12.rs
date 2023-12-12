use std::cell::RefCell;
use std::fmt::format;
use std::rc::Rc;
use std::str::Chars;
use crate::io::read_file_lines;
use crate::problem::Problem;
use regex::Regex;

pub struct DayTwelve {}

fn is_valid(s: &str, groups: &[u32]) -> bool {
    let re_defects = Regex::new(r"(\#+)").unwrap();
    let captured_defects = re_defects.find_iter(s).map(|f| (f.start(), f.end())).collect::<Vec<(usize, usize)>>();
    let b = captured_defects.iter().map(|(start, end)| format!("{:-^1$}", "#", end - start).to_string()).collect::<Vec<String>>();
    let a = groups.iter().map(|n| format!("{:-^1$}", "#", *n as usize).to_string()).collect::<Vec<String>>();
    a.iter().all(|item| b.contains(item))
}

fn map_number_to_str(n: u32) -> String {
    format!(".{}.", "#".repeat(n as usize)).to_string()
}

#[derive(Debug, Copy, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    pub fn from_str(s: &str) -> Self {
        match s {
            "." => Self::Operational,
            "#" => Self::Damaged,
            "?" => Self::Unknown,
            _ => panic!("This cannot happen"),
        }
    }
}

struct Node {
    value: Spring,
    ind: u32,
    group_length: u32,
    children: [Option<Rc<RefCell<Node>>>; 2],
    is_valid: bool,
}

impl Node {
    pub fn advance(&mut self, mut groups: &[u32], line: &str) {
        // println!("I am {:?} this is {} and str length {} with groups {:?}", self.value, self.ind, line.chars().count(), groups);

        if self.ind == line.len() as u32 - 1 {
            // we have reached the end of this line
            if !groups.is_empty() && self.group_length == groups[0] {
                groups = &groups[1..];
            }
            if groups.is_empty() {
                // println!("valid!");
                self.is_valid = true;
            }
            return;
        }

        let mut child_1 = None;
        let mut child_2 = None;

        // child is to be determined
        match self.value {
            Spring::Operational => {
                if line.chars().nth(self.ind as usize + 1).unwrap() == '.' {
                    // if next one is operational
                    child_1 = Some(Rc::new(RefCell::new(Node {
                        value: Spring::Operational,
                        ind: self.ind + 1,
                        group_length: 0,
                        children: [None, None],
                        is_valid: false,
                    })));
                } else if line.chars().nth(self.ind as usize + 1).unwrap() == '#' {
                    // if next one is damaged
                    if groups.is_empty() {
                        // println!("dead because group should be empty but exceeded by fixed #");
                        return;
                    }
                    child_2 = Some(Rc::new(RefCell::new(Node {
                        value: Spring::Damaged,
                        ind: self.ind + 1,
                        group_length: 1,
                        children: [None, None],
                        is_valid: false,
                    })));
                } else {
                    // if next one is unknown
                    child_1 = Some(Rc::new(RefCell::new(Node {
                        value: Spring::Operational,
                        ind: self.ind + 1,
                        group_length: 0,
                        children: [None, None],
                        is_valid: false,
                    })));
                    if !groups.is_empty() {
                        // we have not reached the group length
                        child_2 = Some(Rc::new(RefCell::new(Node {
                            value: Spring::Damaged,
                            ind: self.ind + 1,
                            group_length: 1,
                            children: [None, None],
                            is_valid: false,
                        })));
                    }
                }
            }
            Spring::Damaged => {
                if line.chars().nth(self.ind as usize + 1).unwrap() == '.' {
                    // if next one is operational
                    if self.group_length != groups[0] {
                        // this one is dead
                        // println!("dead because ongoing group killed by fixed .");
                        return;
                    }
                    groups = &groups[1..];
                    child_1 = Some(Rc::new(RefCell::new(Node {
                        value: Spring::Operational,
                        ind: self.ind + 1,
                        group_length: 0,
                        children: [None, None],
                        is_valid: false,
                    })));
                } else if line.chars().nth(self.ind as usize + 1).unwrap() == '#' {
                    // if next one is damaged
                    if self.group_length >= groups[0] {
                        // this one is dead
                        // println!("dead because group exceeded by fixed #");
                        return;
                    }
                    child_2 = Some(Rc::new(RefCell::new(Node {
                        value: Spring::Damaged,
                        ind: self.ind + 1,
                        group_length: self.group_length + 1,
                        children: [None, None],
                        is_valid: false,
                    })));
                } else {
                    // if next one is unknown
                    if self.group_length < groups[0] {
                        // still advancing with damaged group
                        child_2 = Some(Rc::new(RefCell::new(Node {
                            value: Spring::Damaged,
                            ind: self.ind + 1,
                            group_length: self.group_length + 1,
                            children: [None, None],
                            is_valid: false,
                        })));
                    } else if self.group_length == groups[0] {
                        // we have reached the group length
                        groups = &groups[1..];
                        child_1 = Some(Rc::new(RefCell::new(Node {
                            value: Spring::Operational,
                            ind: self.ind + 1,
                            group_length: 0,
                            children: [None, None],
                            is_valid: false,
                        })));
                    }
                }
            }
            Spring::Unknown => {
                // launch two children and remember that we are still at character = ind = 0
                assert_eq!(self.ind, 0);
                child_1 = Some(Rc::new(RefCell::new(Node {
                    value: Spring::Operational,
                    ind: 0,
                    group_length: 0,
                    children: [None, None],
                    is_valid: false,
                })));
                child_2 = Some(Rc::new(RefCell::new(Node {
                    value: Spring::Damaged,
                    ind: 0,
                    group_length: 1,
                    children: [None, None],
                    is_valid: false,
                })));
            }
        }
        self.children = [child_1, child_2];
        for child_option in self.children.iter_mut() {
            if let Some(child) = child_option {
                child.borrow_mut().advance(groups, line)
            }
        }
    }

    fn walk(&self, mut sum: u32) -> u32 {
        // dbg!(&sum);
        for child_option in self.children.iter() {
            if let Some(child) = child_option {
                if child.borrow().is_valid {
                    sum += 1;
                } else {
                    sum = child.borrow().walk(sum);
                }
            }
        }
        sum
    }
}


impl Problem for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut sum = 0;

        for line in contents {
            let parts = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            let data = parts[0].as_str();
            let groups = parts[1].as_str();

            let groups = groups.split(",").map(|u| u.parse::<u32>().unwrap()).collect::<Vec<u32>>();


            let root_spring = Spring::from_str(&data.chars().next().unwrap().to_string());
            // println!("starting with {}", data);
            // dbg!(&root_spring);
            let group_length = match root_spring {
                Spring::Damaged => { 1 }
                _ => { 0 }
            };
            let mut root = Node {
                value: root_spring,
                ind: 0,
                group_length,
                children: [None, None],
                is_valid: false,
            };
            root.advance(&groups, data);

            let walk = root.walk(0);
            sum += walk;
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        dbg!(&contents);
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}