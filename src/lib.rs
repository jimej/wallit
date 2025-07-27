#![allow(dead_code, unused)]
use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    Aes256GcmSiv, Nonce,
};
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
    Show {
        #[arg(short, long)]
        table: String,
        // #[arg(short, long)]
        // all: bool,
        #[arg(long, short = 's')]
        like: Option<String>,
        #[arg(short, long)]
        company: Option<String>,
        #[arg(short, long)]
        limit: Option<i64>,
    },
    Add(SharedArgs), /*{
                         // #[arg(short, long)]
                         // table: String,
                         // #[arg(short, long)]
                         company: String,
                         // #[arg(short, long)]
                         // value: Option<String>,
                         #[arg(short, long)]
                         login: Option<String>,
                         #[arg(short, long)]
                         password: Option<String>,
                         #[arg(long)]
                         url: Option<String>,
                         #[arg(short, long)]
                         email: Option<String>,
                         #[arg(short, long)]
                         description: Option<String>,
                         // #[clap(trailing_var_arg=true)]
                         // remaining: Option<Vec<String>>,
                     }*/

    Update(SharedArgs), /*{

                            #[arg(short, long)]
                            company: String,
                            #[arg(short, long)]
                            value: Option<String>,
                            #[arg(short, long)]
                            login: Option<String>,
                            #[arg(short, long)]
                            password: Option<String>,
                            #[arg(long)]
                            url: Option<String>,
                            #[arg(short, long)]
                            email: Option<String>,
                            #[arg(short, long)]
                            description: Option<String>,
                            #[clap(trailing_var_arg=true)]
                            remaining: Option<Vec<String>>,
                        }*/

    Rotate {
        // only for logins, so no tbl
        #[arg(short, long)]
        company: String,
        #[arg(short, long)]
        value: Option<String>,
    },

    Delete {
        #[arg(short, long)]
        company: String,
    }, // no delete or modify subcommand; everything is audited
       // for a book, will demo how to do it.
}

#[derive(clap::Args, Debug)]
pub struct SharedArgs {
    #[arg(short, long)]
    pub company: String,
    // #[arg(short, long)]
    // value: Option<String>,
    #[arg(short, long)]
    pub login: Option<String>,
    #[arg(short, long)]
    pub password: Option<String>,
    #[arg(short, long)]
    pub url: Option<String>,
    #[arg(short, long)]
    pub email: Option<String>,
    #[arg(short, long)]
    pub description: Option<String>,
}

pub mod schema;
pub mod table_ops;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub fn get_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("could not build connection pool")
}

pub struct SecretGenerator; // for rotation
