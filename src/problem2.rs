use crate::requester;

// post the solution to /solve2
pub fn solve_and_post(url: &str) -> String {
    match requester::post(&format!("{}/problem2/solve", url), &solve(url)) {
        Ok(answer) => answer,
        Err(error)  => error.to_string(),
    }
}

// =================BEGIN CODING=================

fn get_items(url: &str) -> Vec<String> {
    // match the result from the api call, it is Ok, or an Error
    match requester::get(url) {
        Ok(raw_input) => raw_input.split("\n").map(|s| String::from(s)).collect::<Vec<_>>(),
        Err(error)     => panic!("{}", error),
    }
}

fn get_ingredients(url: &str, pizza: &str) -> Vec<String> {
    // match the result from the api call, it is Ok, or an Error
    match requester::get_with_param(&format!("{url}/ingredients"), vec![("pizza", pizza)]) {
        Ok(raw_input) => raw_input.split("\n").map(|s| String::from(s)).collect::<Vec<_>>(),
        Err(error)     => panic!("{}", error),
    }
}

fn count_ingredients(ingredients: Vec<String>, storage: &Vec<String>) -> i32 {
    let mut counts = ingredients.iter().map(|_| 0).collect::<Vec<_>>();
    for item in storage {
        if let Some(position) = ingredients.iter().position(|i| i == item) {
            counts[position] += 1;
        }
    }
    return *counts.iter().min().unwrap()
}

fn solve(url: &str) -> Vec<i32> {
    let storage = get_items(&format!("{}/problem2/input", url));
    let menu = get_items(&format!("{}/menu", url));
    menu
        .iter().map(|p| get_ingredients(url, p))
        .map(|ingredients| count_ingredients(ingredients, &storage)).collect()
}