//use std::io::prelude::*;
// use crate::{
//     Error
// };
use serde::{Deserialize};
use std::time::Duration;
use std::net::TcpStream;
use reqwest::{blocking::Client, redirect};

mod error;
pub use error::Error;

#[derive(Deserialize, Debug)]
struct CrtShResult {
    /// Example: "api.kerkour.com\\nvelib.kerkour.com"
    name_value: String
}

fn main() -> Result<(), anyhow::Error> {

    let res =
    vec!(CrtShResult { name_value: "api.kerkour.com   \\n   velib.kerkour.com".to_owned() }, 
         CrtShResult { name_value: "another.domain".to_owned() });

    println!("Res: {:#?}", res_to_subdomains(res));
    Ok(())
}

fn res_to_subdomains(results: Vec<CrtShResult>) -> Vec<String> {
    results.iter()
    .map(|res| into_names(&res.name_value))
    .flatten()
    .map(|s| s.to_string())
    .collect()
}

fn into_names(name_value: &str) -> Vec<&str> {
    name_value.split("\\n")
    .map(|s: &str| s.trim())
    .collect()
}

fn fetch_subdomains(domain: &str) -> Result<Vec<CrtShResult>, Error> {
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

#[allow(dead_code)]
fn is_port_open(host: &str, port: u16) -> bool {
    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(_) => true,
        Err(_) => false
    }
}