use serde::Deserialize;
use crate::requester;

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
        Err(error)  => error.to_string(),
    }
}

// =================BEGIN CODING=================

#[derive(Deserialize)]
enum Size {
    Small,
    Medium,
    Large,
    American
}

impl Size {
    fn get_size(&self) -> i32 {
        match self {
            Size::Small => 25,
            _ => todo!("fill the rest of the match arms"),
        }
    }

    fn from_str(size_as_string: &str) -> Result<Size, serde_json::Error> {
        serde_json::from_str(size_as_string)
    }
}

fn parse_input(raw_input: String) -> Vec<Size> {
    // you can use the from_str implementation from Size to parse each size to a Size object
    todo!("parse the input into a vector of sizes")
}

fn solve(url: &str) -> i32 {
    let input = parse_input(get_raw_input(url));
    // you can map over the input with the .iter().map()
    // in the map function you can give a lambda to parse each Size to an integer
    // also, the .sum function can come in handy
    todo!("device a solution for problem 1")
}