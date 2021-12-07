#![feature(never_type)]

extern crate tokio;
extern crate toml;

use std::fmt::{Display, Formatter, Debug};
use serde::Deserialize;
use clap::Clap;

mod server;
mod ddc;
mod cli;

pub type Result<T> = core::result::Result<T,SwitcherError>;

#[tokio::main]
async fn main() {
    use crate::cli::Opts;

    let opts: Opts = crate::cli::Opts::parse();

    println!("Monitor switcher started");

    let cfg = read_config(&opts.config);

    let env = match (cfg.is_development(), opts.force_dev, opts.force_prod) {
        (_,Some(_),Some(_)) => {panic!("Can't use force both dev and prod!");},
        (_,Some(true),_) => cfg.development.as_ref().unwrap(),
        (_,_,Some(true)) => cfg.production.as_ref().unwrap(),
        (true,_,_) => cfg.development.as_ref().unwrap(),
        (false,_,_) => cfg.production.as_ref().unwrap(),
    };

    match cfg.is_development() {
        true => println!("Using environment development"),
        false => println!("Using environment production"),
    }

    crate::server::start(&env.ip,env.port).await
        .expect("Starting server failed");
}

pub enum SwitcherError {
    IoError(std::io::ErrorKind),
    ExitCode(i32),
    DDCError(String)
}

impl Display for SwitcherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use SwitcherError::*;

        match &self {
            IoError(e) => write!(f,"{:?}",e),
            ExitCode(num) => write!(f,"DDC command failed with exit code {}",num),
            DDCError(text) => write!(f,"DDC failed, {}", text)
        }
    }
}

impl From<std::io::Error> for SwitcherError {
    fn from(e: std::io::Error) -> Self {
        SwitcherError::IoError(e.kind())
    }
}

impl Debug for SwitcherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",&self)
    }
}

fn read_config(path: &str) -> Config{
    use std::fs;

    toml::from_str(&fs::read_to_string(path)
        .expect("Failed to read Switcher.toml"))
        .expect("Invalid Switcher.toml")
}

#[derive(Deserialize)]
pub struct Config {
    development: Option<CEnvironment>,
    production: Option<CEnvironment>,
}

#[derive(Deserialize)]
pub struct CEnvironment {
    ip: String,
    port: u16
}

impl Config {
    pub fn is_development(&self) -> bool{
        if cfg!(debug_assertions) {
            assert!(self.development.is_some());
            true
        }
        else {
            assert!(self.production.is_some());
            false
        }
    }
}