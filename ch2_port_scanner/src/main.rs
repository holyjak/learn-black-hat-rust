//use std::io::prelude::*;
// use crate::{
//     Error
// };
use std::net::TcpStream;

mod error;
pub use error::Error;
mod subdomains;
pub use subdomains::CrtShResult;

fn main() -> Result<(), anyhow::Error> {

    println!("Res: {:#?}", subdomains::fetch_subdomains("kerkour.com"));
    Ok(())
}

#[allow(dead_code)]
fn is_port_open(host: &str, port: u16) -> bool {
    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(_) => true,
        Err(_) => false
    }
}