use std::collections::HashSet;
use crate::{ Error };

use serde::{Deserialize};
use std::time::Duration;
use reqwest::{blocking::Client, redirect};

#[derive(Deserialize, Debug)]
pub struct CrtShResult {
    /// Example: "api.kerkour.com\\nvelib.kerkour.com"
    name_value: String
}

pub fn fetch_subdomains(domain: &str) -> Result<HashSet<String>, Error> {
    let crt_results = fetch_subdomains_int(domain)?;
    Ok(res_to_subdomains(crt_results))
}

fn res_to_subdomains(results: Vec<CrtShResult>) -> HashSet<String> {
    results.iter()
    .map(|res| into_names(&res.name_value))
    .flatten()
    .map(|s| s.to_string())
    .filter(|s| !s.starts_with("*."))
    .collect()
}

fn into_names(name_value: &str) -> Vec<&str> {
    name_value.split("\n")
    .map(|s: &str| s.trim())
    .collect()
}

fn fetch_subdomains_int(domain: &str) -> Result<Vec<CrtShResult>, Error> {
    let client = Client::builder()
                    .redirect(redirect::Policy::limited(4))
                    .timeout(Duration::from_secs(60))
                    .build()?;

    let res = client.get(format!("https://crt.sh/?q=%25.{}", domain))
                .header("Accept", "application/json")
                .send()?
                .json::<Vec<CrtShResult>>()?;
    Ok(res)
}