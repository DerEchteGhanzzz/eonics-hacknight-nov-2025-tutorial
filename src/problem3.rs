use crate::requester;

// post the solution to /solve2
pub fn solve_and_post(url: &str) -> String {
    match requester::post(&format!("{}/problem3/solve", url), &solve(url)) {
        Ok(answer) => answer,
        Err(error)  => error.to_string(),
    }
}

// =================BEGIN CODING=================

fn solve(url: &str) -> i32 {
    /*
    For this problem you might want to use `cargo run --release` to make the calculations a bit quicker
    */
    -1
}