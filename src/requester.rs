use reqwest::Url;
use anyhow::{Error, Result};
use reqwest::blocking::Client;
use serde::Serialize;

pub fn get(url: &str) -> Result<String, Error> {
    let url = Url::parse(url)?;
    Ok(
        Client::new()
        .get(url)
        .send()?
        .text()?
    )
}

pub fn get_with_param(url: &str, param: (&str, &str)) -> Result<String, Error> {
    let url = Url::parse_with_params(url, [param])?;
    let response = Client::new()
        .get(url).send()?.text()?;
    Ok(response)
}


pub fn post<T: Serialize>(url: &str, body: &T) -> Result<String, Error> {
    let url = Url::parse(url)?;
    let response = Client::new()
        .post(url).body(serde_json::to_string(body)?).send()?.text()?;
    Ok(response)
}