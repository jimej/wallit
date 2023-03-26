#![allow(dead_code, unused)]
use clap::{Parser, Subcommand};
use std::{error::Error, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,

    // #[arg(short, long, default_value_t = 1)]
    // pub zcount: u8,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: String,
    },
    Delete {},
}

// pub fn get_args() -> Result<Config, Box<dyn Error>> {
//     let matches = App::new("wallit")
//         .version("0.1.0")
//         .author("jimej")
//         .about("wallet foor secrets")
//         .get_matches();

//     Ok(Config {})
// }
