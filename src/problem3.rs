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
    match requester::get(&format!("{url}/problem3/input")) {
        Ok(raw_input) => raw_input.split("\n").map(|s| String::from(s)).collect::<Vec<_>>(),
        Err(error)     => panic!("{}", error),
    }
}

fn parse_input(input: Vec<String>) -> HashMap<String, HashMap<String, i32>> {
    let mut lines = input.into_iter();
    let binding = lines.next().unwrap();
    let names = binding.split(";").skip(1).collect::<Vec<_>>();
    let mut dist: HashMap<String, HashMap<String, i32>> = HashMap::from_iter(names.iter().map(|h| (String::from(*h), HashMap::new())));
    for line in lines {
        let mut split_line = line.split(";").into_iter();
        let origin = split_line.next().unwrap();
        for (target, distance) in names.iter().zip(split_line.map(|i| i32::from_str_radix(i, 10).unwrap())) {
            dist.get_mut(origin).unwrap().insert(String::from(*target), distance);
        }
    }
    return dist;
}

fn tsp(dist: &HashMap<String, HashMap<String, i32>>, current: i32, visited: Vec<&String>) -> i32 {
    if visited.len() == dist.keys().len() {
        return current + dist.get(*visited.last().unwrap()).unwrap().get(visited[0]).unwrap();
    }
    let mut best = i32::MAX;
    let next = dist.get(*visited.last().unwrap()).unwrap();
    for (target, distance) in next {
        if visited.contains(&target) {
            continue;
        }
        best = best.min(tsp(dist, current + distance, visited.clone().into_iter().chain(vec![target]).collect::<Vec<_>>()))
    }
    return best
}

fn solve(url: &str) -> i32 {
    let dist = parse_input(get_input(url));
    tsp(&dist, 0, vec![dist.keys().next().unwrap()])
}