use crate::requester;

// post the solution to /solve2
pub fn solve_and_post(url: &str) -> String {
    match requester::post(&format!("{}/problem2/solve", url), &solve(url)) {
        Ok(answer) => answer,
        Err(error)  => error.to_string(),
    }
}

fn get_menu(url: &str) -> Vec<String> {
    requester::get_elements(&format!("{url}/menu"))
}


fn get_input(url: &str) -> Vec<String> {
    requester::get_elements(&format!("{url}/problem2/input"))
}


fn get_ingredients(url: &str, pizza: &str) -> Vec<String> {
    // match the result from the api call, it is Ok, or an Error
    match requester::get_with_param(&format!("{url}/ingredients"), ("pizza", pizza)) {
        Ok(raw_input) => raw_input.split("\n").map(|s| String::from(s)).collect::<Vec<_>>(),
        Err(error)     => panic!("{}", error),
    }
}

// =================BEGIN CODING=================

fn solve(url: &str) -> Vec<i32> {
    /*
        You can use the get_ingredients function to perform a GET request with a pizza name as a parameter
        Try to use the map function to map each menu item to its ingredients and then map those ingredients to their occurrances
     */
    let menu = get_menu(url);
    let storage = get_input(url);
    println!("Implement problem 2");
    vec![-1, -1, -1]
}