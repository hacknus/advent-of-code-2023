use std::cmp::Ordering;
use std::collections::HashSet;
use itertools::Itertools;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DaySeven {}

#[derive(Default, Debug, Clone)]
struct Hand {
    name: String,
    bid: u32,
}

impl Hand {
    pub fn contains_one_pair(&self) -> bool {
        let mut nums = vec![0; self.name.len()];
        for (i, c) in self.name.char_indices() {
            for c2 in self.name.chars() {
                if c == c2 {
                    nums[i] += 1;
                }
            }
        }
        *nums.iter().max().unwrap() == 2
    }

    pub fn contains_two_pair(&self) -> bool {
        // first pair:
        let mut first_pair = '0';

        for (i, c) in self.name.char_indices() {
            for (j, c2) in self.name.char_indices() {
                if c == c2 && i != j {
                    first_pair = c.clone();
                }
            }
        }
        let mut nums = vec![1; self.name.len()];
        for (i, c) in self.name.char_indices() {
            for (j, c2) in self.name.char_indices() {
                if c == c2 && i != j && c2 != first_pair {
                    nums[i] += 1;
                }
            }
        }
        *nums.iter().max().unwrap() == 2
    }

    pub fn contains_three_of_a_kind(&self) -> bool {
        let mut nums = vec![0; self.name.len()];
        for (i, c) in self.name.char_indices() {
            for c2 in self.name.chars() {
                if c == c2 {
                    nums[i] += 1;
                }
            }
        }
        *nums.iter().max().unwrap() == 3
    }

    pub fn contains_full_house(&self) -> bool {
        let hashset = self.name.chars().into_iter()
            .collect::<HashSet<_>>();
        hashset.len() == 2
    }

    pub fn contains_four_of_a_kind(&self) -> bool {
        let mut nums = vec![0; self.name.len()];
        for (i, c) in self.name.char_indices() {
            for c2 in self.name.chars() {
                if c == c2 {
                    nums[i] += 1;
                }
            }
        }
        *nums.iter().max().unwrap() == 4
    }

    pub fn contains_five_of_a_kind(&self) -> bool {
        let hashset = self.name.chars().into_iter()
            .collect::<HashSet<_>>();
        hashset.len() == 1
    }

    pub fn contains_five_different_cards(&self) -> bool {
        let hashset = self.name.chars().into_iter()
            .collect::<HashSet<_>>();
        hashset.len() == 5
    }

    pub fn cmp_strings(&self, other: &Hand) -> Ordering {
        let mut a_chars = self.name.chars();
        let mut b_chars = other.name.chars();

        loop {
            match (a_chars.next(), b_chars.next()) {
                (Some(ac), Some(bc)) => {
                    // Compare characters based on rules
                    let order = if ac.is_alphabetic() && bc.is_alphabetic() {
                        match ac.to_ascii_uppercase() {
                            'A' => {
                                match bc.to_ascii_uppercase() {
                                    'A' => Ordering::Equal,
                                    'K' => Ordering::Less,
                                    'Q' => Ordering::Less,
                                    'J' => Ordering::Less,
                                    'T' => Ordering::Less,
                                    _ => Ordering::Less
                                }
                            }
                            'K' => {
                                match bc.to_ascii_uppercase() {
                                    'A' => Ordering::Greater,
                                    'K' => Ordering::Equal,
                                    'Q' => Ordering::Less,
                                    'J' => Ordering::Less,
                                    'T' => Ordering::Less,
                                    _ => Ordering::Less
                                }
                            }
                            'Q' => {
                                match bc.to_ascii_uppercase() {
                                    'A' => Ordering::Greater,
                                    'K' => Ordering::Greater,
                                    'Q' => Ordering::Equal,
                                    'J' => Ordering::Less,
                                    'T' => Ordering::Less,
                                    _ => Ordering::Less
                                }
                            }
                            'J' => {
                                match bc.to_ascii_uppercase() {
                                    'A' => Ordering::Greater,
                                    'K' => Ordering::Greater,
                                    'Q' => Ordering::Greater,
                                    'J' => Ordering::Equal,
                                    'T' => Ordering::Less,
                                    _ => Ordering::Less
                                }
                            }
                            'T' => {
                                match bc.to_ascii_uppercase() {
                                    'A' => Ordering::Greater,
                                    'K' => Ordering::Greater,
                                    'Q' => Ordering::Greater,
                                    'J' => Ordering::Greater,
                                    'T' => Ordering::Equal,
                                    _ => Ordering::Less
                                }
                            }
                            _ => { Ordering::Less }
                        }
                    } else if ac.is_digit(10) && bc.is_digit(10) {
                        bc.cmp(&ac)
                    } else if ac.is_digit(10) {
                        Ordering::Greater
                    } else if bc.is_digit(10) {
                        Ordering::Less
                    } else {
                        // If both characters are non-alphabetic and non-numeric, consider them equal
                        Ordering::Equal
                    };

                    if order != Ordering::Equal {
                        return order;
                    }
                }
                (Some(_), None) => return Ordering::Greater,
                (None, Some(_)) => return Ordering::Less,
                (None, None) => return Ordering::Equal,
            }
        }
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.contains_five_of_a_kind() {
            if other.contains_five_of_a_kind() {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else if self.contains_four_of_a_kind() {
            if other.contains_five_of_a_kind() {
                Ordering::Greater
            } else if other.contains_four_of_a_kind() {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else if self.contains_full_house() {
            if other.contains_five_of_a_kind() {
                Ordering::Greater
            } else if other.contains_four_of_a_kind() {
                Ordering::Greater
            } else if other.contains_full_house() {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else if self.contains_three_of_a_kind() {
            if other.contains_five_of_a_kind() {
                Ordering::Greater
            } else if other.contains_four_of_a_kind() {
                Ordering::Greater
            } else if other.contains_full_house() {
                Ordering::Greater
            } else if other.contains_three_of_a_kind() {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else if self.contains_two_pair() {
            if other.contains_five_of_a_kind() {
                Ordering::Greater
            } else if other.contains_four_of_a_kind() {
                Ordering::Greater
            } else if other.contains_full_house() {
                Ordering::Greater
            } else if other.contains_three_of_a_kind() {
                Ordering::Greater
            } else if other.contains_two_pair() {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else if self.contains_one_pair() {
            if other.contains_five_of_a_kind() {
                Ordering::Greater
            } else if other.contains_four_of_a_kind() {
                Ordering::Greater
            } else if other.contains_full_house() {
                Ordering::Greater
            } else if other.contains_three_of_a_kind() {
                Ordering::Greater
            } else if other.contains_two_pair() {
                Ordering::Greater
            } else if other.contains_one_pair() {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else {
            if other.contains_five_different_cards() {
                self.cmp_strings(&other)
            } else {
                Ordering::Greater
            }
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Hand2 {
    name: String,
    bid: u32,
}

impl Hand2 {
    pub fn contains_one_pair(&self, num_joker: u8) -> bool {
        if num_joker > 0
        {
            return true;
        }
        let mut nums = vec![0; self.name.len()];
        for (i, c) in self.name.char_indices() {
            for c2 in self.name.chars() {
                if c == c2 && c != 'J' {
                    nums[i] += 1;
                }
            }
        }
        *nums.iter().max().unwrap() == 2
    }

    pub fn contains_two_pair(&self, num_joker: u8) -> bool {
        if num_joker > 0 && self.contains_one_pair(0)
        {
            return true;
        }
        // first pair:
        let mut first_pair = '0';

        for (i, c) in self.name.char_indices() {
            for (j, c2) in self.name.char_indices() {
                if c == c2 && i != j && c != 'J' {
                    first_pair = c.clone();
                }
            }
        }
        let mut nums = vec![1; self.name.len()];
        for (i, c) in self.name.char_indices() {
            for (j, c2) in self.name.char_indices() {
                if c == c2 && i != j && c2 != first_pair && c != 'J' {
                    nums[i] += 1;
                }
            }
        }
        *nums.iter().max().unwrap() == 2
    }

    pub fn contains_three_of_a_kind(&self, num_joker: u8) -> bool {
        if num_joker >= 2 {
            return true;
        }
        if num_joker == 1 && self.contains_one_pair(0)
        {
            return true;
        }
        let mut nums = vec![0; self.name.len()];
        for (i, c) in self.name.char_indices() {
            for c2 in self.name.chars() {
                if c == c2 && c != 'J' {
                    nums[i] += 1;
                }
            }
        }
        *nums.iter().max().unwrap() == 3
    }

    pub fn contains_full_house(&self, num_joker: u8) -> bool {
        if num_joker >= 3
        {
            return true;
        }
        if num_joker >= 2 && self.contains_one_pair(0)
        {
            return true;
        }
        if num_joker >= 2 && self.contains_three_of_a_kind(0)
        {
            return true;
        }
        if num_joker >= 1 && self.contains_two_pair(0)
        {
            return true;
        }
        let hashset = self.name.chars().into_iter()
            .collect::<HashSet<_>>();
        hashset.len() == 2
    }

    pub fn contains_four_of_a_kind(&self, num_joker: u8) -> bool {
        if num_joker >= 3
        {
            return true;
        }
        if num_joker >= 2 && self.contains_one_pair(0)
        {
            return true;
        }
        if num_joker >= 1 && self.contains_three_of_a_kind(0)
        {
            return true;
        }
        if num_joker >= 1 && self.contains_full_house(0)
        {
            return true;
        }
        let mut nums = vec![0; self.name.len()];
        for (i, c) in self.name.char_indices() {
            for c2 in self.name.chars() {
                if c == c2 && c != 'J' {
                    nums[i] += 1;
                }
            }
        }
        *nums.iter().max().unwrap() == 4
    }

    pub fn contains_five_of_a_kind(&self, num_joker: u8) -> bool {
        if num_joker >= 4
        {
            return true;
        }
        if num_joker >= 3 && self.contains_one_pair(0)
        {
            return true;
        }
        if num_joker >= 2 && self.contains_three_of_a_kind(0)
        {
            return true;
        }
        if num_joker >= 1 && self.contains_four_of_a_kind(0)
        {
            return true;
        }
        let hashset = self.name.chars().into_iter()
            .collect::<HashSet<_>>();
        hashset.len() == 1
    }

    pub fn contains_five_different_cards(&self) -> bool {
        if self.name.contains("J") {
            return false;
        }
        let hashset = self.name.chars().into_iter()
            .collect::<HashSet<_>>();
        hashset.len() == 5
    }

    pub fn cmp_strings(&self, other: &Hand2) -> Ordering {
        let mut a_chars = self.name.chars();
        let mut b_chars = other.name.chars();

        loop {
            match (a_chars.next(), b_chars.next()) {
                (Some(ac), Some(bc)) => {
                    // Compare characters based on rules
                    let order = if ac.is_alphabetic() && bc.is_alphabetic() {
                        match ac.to_ascii_uppercase() {
                            'A' => {
                                match bc.to_ascii_uppercase() {
                                    'A' => Ordering::Equal,
                                    'K' => Ordering::Less,
                                    'Q' => Ordering::Less,
                                    'T' => Ordering::Less,
                                    'J' => Ordering::Less,
                                    _ => Ordering::Less
                                }
                            }
                            'K' => {
                                match bc.to_ascii_uppercase() {
                                    'A' => Ordering::Greater,
                                    'K' => Ordering::Equal,
                                    'Q' => Ordering::Less,
                                    'T' => Ordering::Less,
                                    'J' => Ordering::Less,
                                    _ => Ordering::Less
                                }
                            }
                            'Q' => {
                                match bc.to_ascii_uppercase() {
                                    'A' => Ordering::Greater,
                                    'K' => Ordering::Greater,
                                    'Q' => Ordering::Equal,
                                    'T' => Ordering::Less,
                                    'J' => Ordering::Less,
                                    _ => Ordering::Less
                                }
                            }
                            'T' => {
                                match bc.to_ascii_uppercase() {
                                    'A' => Ordering::Greater,
                                    'K' => Ordering::Greater,
                                    'Q' => Ordering::Greater,
                                    'T' => Ordering::Equal,
                                    'J' => Ordering::Less,
                                    _ => Ordering::Less
                                }
                            }
                            'J' => {
                                match bc.to_ascii_uppercase() {
                                    'A' => Ordering::Greater,
                                    'K' => Ordering::Greater,
                                    'Q' => Ordering::Greater,
                                    'T' => Ordering::Greater,
                                    'J' => Ordering::Equal,
                                    _ => Ordering::Less
                                }
                            }
                            _ => { Ordering::Less }
                        }
                    } else if ac.is_digit(10) && bc.is_digit(10) {
                        bc.cmp(&ac)
                    } else if ac.is_digit(10) {
                        if bc.to_ascii_uppercase() == 'J' {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    } else if bc.is_digit(10) {
                        if ac.to_ascii_uppercase() == 'J' {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    } else {
                        // If both characters are non-alphabetic and non-numeric, consider them equal
                        Ordering::Equal
                    };

                    if order != Ordering::Equal {
                        return order;
                    }
                }
                (Some(_), None) => return Ordering::Greater,
                (None, Some(_)) => return Ordering::Less,
                (None, None) => return Ordering::Equal,
            }
        }
    }
}

impl Eq for Hand2 {}

impl PartialEq<Self> for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd<Self> for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let num_jokers = self.name.chars().filter(|&n| n == 'J').count() as u8;
        let num_other_jokers = other.name.chars().filter(|&n| n == 'J').count() as u8;
        if self.contains_five_of_a_kind(num_jokers) {
            if other.contains_five_of_a_kind(num_other_jokers) {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else if self.contains_four_of_a_kind(num_jokers) {
            if other.contains_five_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_four_of_a_kind(num_other_jokers) {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else if self.contains_full_house(num_jokers) {
            if other.contains_five_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_four_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_full_house(num_other_jokers) {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else if self.contains_three_of_a_kind(num_jokers) {
            if other.contains_five_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_four_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_full_house(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_three_of_a_kind(num_other_jokers) {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else if self.contains_two_pair(num_jokers) {
            if other.contains_five_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_four_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_full_house(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_three_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_two_pair(num_other_jokers) {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else if self.contains_one_pair(num_jokers) {
            if other.contains_five_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_four_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_full_house(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_three_of_a_kind(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_two_pair(num_other_jokers) {
                Ordering::Greater
            } else if other.contains_one_pair(num_other_jokers) {
                self.cmp_strings(&other)
            } else {
                Ordering::Less
            }
        } else {
            if other.contains_five_different_cards() {
                self.cmp_strings(&other)
            } else {
                Ordering::Greater
            }
        }
    }
}

impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut hands = vec![];
        for line in contents {
            let parts = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            let hand = Hand {
                name: parts[0].clone(),
                bid: parts[1].parse::<u32>().unwrap(),
            };
            hands.push(hand);
        }
        hands.sort_by(|a, b| {
            b.cmp(&a)
        });
        let winnings = hands.iter().enumerate().map(|(rank, hand)| {
            (rank as u32 + 1) * hand.bid
        }).sum::<u32>();

        format!("{}", winnings)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut hands = vec![];
        for line in contents {
            let parts = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            let hand = Hand2 {
                name: parts[0].clone(),
                bid: parts[1].parse::<u32>().unwrap(),
            };
            hands.push(hand);
        }
        hands.sort_by(|a, b| {
            b.cmp(&a)
        });
        let winnings = hands.iter().enumerate().map(|(rank, hand)| {
            (rank as u32 + 1) * hand.bid
        }).sum::<u32>();
        format!("{}", winnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}