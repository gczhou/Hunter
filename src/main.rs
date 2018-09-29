extern crate serde;
extern crate serde_json;
extern crate hyper;

// cli crate
extern crate getopts;
extern crate libc;
extern crate time;

use std::env;
use std::io::{self, Write};

mod case;
mod cli;
mod core;

fn main() {
    println!("Hello, world!");

    cli::Command::run();
}
