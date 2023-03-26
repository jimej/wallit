#![allow(dead_code, unused)]
use clap::{Parser};
use std::error::Error;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

// pub fn get_args() -> Result<Config, Box<dyn Error>> {
//     let matches = App::new("wallit")
//         .version("0.1.0")
//         .author("jimej")
//         .about("wallet foor secrets")
//         .get_matches();

//     Ok(Config {})
// }
