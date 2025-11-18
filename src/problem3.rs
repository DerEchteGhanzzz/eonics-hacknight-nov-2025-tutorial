use crate::requester;

// post the solution to /solve2
pub fn solve_and_post(url: &str) -> String {
    let solution = solve(url);
    println!("Answer 3: {}", solution);
    match requester::post(&format!("{}/problem3/solve", url), &solution) {
        Ok(answer) => answer,
        Err(error)  => error.to_string(),
    }
}

// =================BEGIN CODING=================

fn solve(url: &str) -> i32 {
    /*
    For this problem you might want to use `cargo run --release` to make the calculations a bit quicker

    Calculate the minimum travel time for our courrier to visit every location and come back here again.
    Use the api /problem3/locations for the locations to visit, and /problem3/from-to for the travel times between them.
    */
    println!("Implement problem 3");
    -1
}