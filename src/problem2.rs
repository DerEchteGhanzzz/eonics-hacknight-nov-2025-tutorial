use crate::requester;

fn get_raw_menu(url: &str) -> String {
    // match the result from the api call, it is Ok, or an Error
    match requester::get(&format!("{}/problem2/menu", url)) {
        Ok(raw_input) => raw_input,
        Err(error)     => panic!("{}", error),
    }
}

// post the solution to /solve2
pub fn solve_and_post(url: &str) -> String {
    match requester::post(&format!("{}/problem2/solve", url), &solve(url)) {
        Ok(answer) => answer,
        Err(error)  => error.to_string(),
    }
}

// =================BEGIN CODING=================

fn get_raw_ingredients(url: &str, pizza_name: &str) -> String {
    match requester::get_with_param(&format!("{}/problem2/ingredients", url), ("pizza", pizza_name)) {
        Ok(ingredients_raw) => ingredients_raw,
        Err(error)           => panic!("{:?}", error),
    }
}

fn solve(url: &str) -> i32 {
    let raw_menu = get_raw_menu(url);
    /*
    you can use serde_json::from_str() to parse the raw_menu to a string
    for each menu item, you'll need to make an api call to /ingredients?pizza=menu_item
    to get its ingredients
    
    use requester::get_with_param(self.url, param_name_and_value: (&str, &str))
    to make a request with a parameter
    */

    todo!("device a solution for problem 2")
}