use serde::Deserialize;
use crate::requester;

#[derive(Deserialize)]
enum Size {
    Small,
    Medium,
    Large,
    American
}

fn get_raw_input(url: &str) -> String {
    match requester::get(&format!("{}/problem1/input", url)) {
        Ok(input) => input,
        Err(error) => panic!("{:?}", error),
    }
}

// post the solution to /solve1
pub fn solve_and_post(url: &str) -> String {
    match requester::post(&format!("{}/problem1/solve", url), &solve(url)) {
        Ok(answer) => answer,
        Err(error)  => panic!("{:?}", error),
    }
}

// =================BEGIN CODING=================

impl Size {
    fn get_size(&self) -> i32 {
        match self {
            Size::Small => 25,
            Size::Medium => 29,
            Size::Large => 35,
            Size::American => 90,
        }
    }

    fn from_str(size_as_string: &str) -> Size {
        match size_as_string {
            "Small" => Size::Small,
            "Medium" => Size::Medium,
            "Large" => Size::Large,
            "American" => Size::American,
            _ => panic!("Not a size!"),
        }
    }
}

const PI: i32 = 3;

fn solve(url: &str) -> i32 {
    get_raw_input(url).split("\r\n")
        .map(|raw_size| Size::from_str(raw_size).get_size())
        .fold(0, |acc, z| acc + PI * z * z)
}