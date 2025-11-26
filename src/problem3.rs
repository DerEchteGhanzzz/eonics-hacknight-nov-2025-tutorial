use std::{collections::HashMap, i32};

use crate::requester;

// post the solution to /solve2
pub fn solve_and_post(url: &str) -> String {
    match requester::post(&format!("{}/problem3/solve", url), &solve(url)) {
        Ok(answer) => answer,
        Err(error)  => error.to_string(),
    }
}

// =================BEGIN CODING=================

fn get_input(url: &str) -> Vec<String> {
    // match the result from the api call, it is Ok, or an Error
    match requester::get(&format!("{url}/problem3/locations")) {
        Ok(raw_input) => raw_input.split("\n").map(|s| String::from(s)).collect::<Vec<_>>(),
        Err(error)     => panic!("{}", error),
    }
}

fn get_from_to(url: &str, s: &str, t: &str) -> u32 {
    // match the result from the api call, it is Ok, or an Error
    match requester::get_with_param(&format!("{url}/problem3/from-to"), vec![("from", s), ("to", t)]) {
        Ok(raw_input) => {
            u32::from_str_radix(&raw_input, 10).unwrap()},
        Err(error)     => panic!("{}", error),
    }
}

fn parse_input(url: &str, names: Vec<String>) -> Vec<Vec<i32>> {
    Vec::from_iter(names.iter().map(|s|  
        Vec::from_iter(
            names.iter().map(|t| get_from_to(url, s, t) as i32)
            )
        )
    )
}

fn tsp(dist: &Vec<Vec<i32>>, current: i32, visited: Vec<usize>, ub: i32) -> i32 {
    if visited.len() == dist.len() {
        return current + dist[visited[0]][*visited.last().unwrap()];
    }
    let mut best = i32::MAX;
    if current >= ub {
        return best;
    }
    let next = dist.get(*visited.last().unwrap()).unwrap();
    for (target, distance) in next.iter().enumerate() {
        if visited.contains(&target) {
            continue;
        }
        best = best.min(tsp(dist, 
            current + distance, 
            visited.clone().into_iter().chain(vec![target]).collect::<Vec<_>>(), 
            ub.min(best))
        )
    }
    return best
}

fn solve(url: &str) -> i32 {
    let dist = parse_input(url, get_input(url));
    tsp(&dist, 0, vec![0], i32::MAX)
}