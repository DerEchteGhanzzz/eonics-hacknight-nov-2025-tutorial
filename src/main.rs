use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

mod requester;
mod problem1;
mod problem2;
mod problem3;

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}

fn main() -> Result<()> {
    let url: &str = "http://rustykrab.nl";
    
    println!("Answer 1:\n{}", problem1::solve_and_post(url));
    println!("Answer 2:\n{}", problem2::solve_and_post(url));
    println!("Answer 3:\n{}", problem3::solve_and_post(url));
    
    Ok(())
}