mod io;
mod problem;
#[allow(unused)]
mod days;

use std::time::Instant;
use problem::Problem;
use days::day_1::DayOne;
use days::day_2::DayTwo;
use days::day_3::DayThree;
use days::day_4::DayFour;
use days::day_5::DayFive;
use days::day_6::DaySix;
use days::day_7::DaySeven;
use days::day_8::DayEight;
use days::day_9::DayNine;
use days::day_10::DayTen;
use days::day_11::DayEleven;
use days::day_12::DayTwelve;
use days::day_13::DayThirteen;
use days::day_14::DayFourteen;
use days::day_15::DayFifthteen;
use days::day_16::DaySixteen;
use days::day_17::DaySeventeen;
use days::day_18::DayEighteen;
use days::day_19::DayNineteen;
use days::day_20::DayTwenty;
use days::day_21::DayTwentyOne;
use days::day_22::DayTwentyTwo;
use days::day_23::DayTwentyThree;
use days::day_24::DayTwentyFour;
use days::day_25::DayTwentyFive;
use chrono::Datelike;

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne {})),
        2 => Some(Box::new(DayTwo {})),
        3 => Some(Box::new(DayThree {})),
        4 => Some(Box::new(DayFour {})),
        5 => Some(Box::new(DayFive {})),
        6 => Some(Box::new(DaySix {})),
        7 => Some(Box::new(DaySeven {})),
        8 => Some(Box::new(DayEight {})),
        9 => Some(Box::new(DayNine {})),
        10 => Some(Box::new(DayTen {})),
        11 => Some(Box::new(DayEleven {})),
        12 => Some(Box::new(DayTwelve {})),
        13 => Some(Box::new(DayThirteen {})),
        14 => Some(Box::new(DayFourteen {})),
        15 => Some(Box::new(DayFifthteen {})),
        16 => Some(Box::new(DaySixteen {})),
        17 => Some(Box::new(DaySeventeen {})),
        18 => Some(Box::new(DayEighteen {})),
        19 => Some(Box::new(DayNineteen {})),
        20 => Some(Box::new(DayTwenty {})),
        21 => Some(Box::new(DayTwentyOne {})),
        22 => Some(Box::new(DayTwentyTwo {})),
        23 => Some(Box::new(DayTwentyThree {})),
        24 => Some(Box::new(DayTwentyFour {})),
        25 => Some(Box::new(DayTwentyFive {})),
        _ => None
    }
}

fn main() {
    let current_date = chrono::Utc::now();
    let day = current_date.day();
    match day_to_problem(day as usize) {
        None => {
            println!("No problem for day {day}...");
        }
        Some(problem) => {

            let start = Instant::now();
            let answer_one = problem.part_one(format!("input/puzzle_{day}_1.txt").as_str());
            println!("solving task one took {:?}",start.elapsed());

            let start = Instant::now();
            let answer_two = problem.part_two(format!("input/puzzle_{day}_2.txt").as_str());
            println!("solving task two took {:?}",start.elapsed());

            println!("Answer of Task Day {day}/1:");
            println!("{answer_one}\n");
            println!("Answer of Task Day {day}/2:");
            println!("{answer_two}");
        }
    }
}
