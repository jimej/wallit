#![allow(dead_code, unused)]
use aes_gcm_siv::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
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
    Add {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: String,
    },
    Delete {},
}

pub struct CipherMaterial {
    cipher: Aes256GcmSiv,
    nonce: Nonce,
}

impl CipherMaterial {
    pub fn new() -> Self {
        let key = Aes256GcmSiv::generate_key(&mut OsRng);
        let cipher = Aes256GcmSiv::new(&key);
        let nonce = Aes256GcmSiv::generate_nonce(&mut OsRng); // 96-bits; unique per message
        CipherMaterial { cipher, nonce }
    }

    pub fn encrypt(&self, input: &str) -> Result<Vec<u8>, aes_gcm_siv::Error> {
        let ciphertext = self
            .cipher
            .encrypt(&self.nonce, input.as_bytes().as_ref())?;
        Ok(ciphertext)
    }

    pub fn decrypt(&self, ciphertext: Vec<u8>) -> Result<Vec<u8>, aes_gcm_siv::Error> {
        let plaintext = self.cipher.decrypt(&self.nonce, ciphertext.as_ref())?; // aes_gcm_siv::Error is not std::error::Error
                                                                                // assert_eq!(&plaintext, b"plaintext message");
        Ok(plaintext)
    }
}

impl Default for CipherMaterial {
    fn default() -> Self {
        Self::new()
    }
}

pub mod schema;
mod table_ops;

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
