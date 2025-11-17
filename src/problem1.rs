use serde::Deserialize;
use crate::requester;

#[derive(Deserialize)]
enum Size {
    Small,
    Medium,
    Large,
    American
}

fn get_input(url: &str) -> Vec<String> {
    requester::get_elements(&format!("{url}/problem1/input"))
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
            _           => todo!("finish the match expression by filling in the rest of the sizes"),
        }
    }

    fn from_str(size_as_string: &str) -> Size {
        match size_as_string {
            "Small" => Size::Small,
            _       => todo!("finish the match expression by filling in the rest of the names")
        }
    }
}

fn solve(url: &str) -> i32 {
    /*
        This exersize is to show off the Enums rust have.
        You don't need to use the enum declared above, but to get a better understanding
        of how pattern matching works in rust, you can try to finish the code

        use the get_input(url) function to get the input from problem 1
        try to use the .iter().map() functions to map each element from the input to its size and calculate the area
     */
    println!("Implement problem 1");
    -1
}