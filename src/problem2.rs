use crate::requester;

// post the solution to /solve2
pub fn solve_and_post(url: &str) -> String {
    let solution = solve(url);
    println!("Answer 2: {:?}", solution);
    match requester::post(&format!("{}/problem2/solve", url), &solution) {
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
    match requester::get_with_param(&format!("{url}/ingredients"), vec![("pizza", pizza)]) {
        Ok(raw_input) => raw_input.split("\n").map(|s| String::from(s)).collect::<Vec<_>>(),
        Err(error)     => panic!("{}", error),
    }
}

// =================BEGIN CODING=================

fn solve(url: &str) -> Vec<i32> {
    /*
        Calculate for each pizza type <u>separately</u> how many can be made with the ingredients in the storage.
        Send your result as a formatted vector like: `[0, 0, 0, ..]` (use: `format!(\"{{:?}}\", result)` to turn your vector into a string) to /problem2/answer. 
        Each index in the list corresponds with each item on our menu.

        You can use the get_ingredients function to perform a GET request with a pizza name as a parameter
        Try to use the map function to map each menu item to its ingredients and then map those ingredients to their occurrances
     */
    let menu = get_menu(url);
    let storage = get_input(url);
    println!("Implement problem 2");
    vec![-1, -1, -1]
}