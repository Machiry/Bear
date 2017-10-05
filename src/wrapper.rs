// Copyright (c) 2017 László Nagy
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::process;
mod parameters;

fn main() {
    let build: Vec<_> = env::args().collect();
    let args = &build[1..];
    let config = parameters::read().unwrap();

    let command = process::Command::new(config.cc)
        .args(args)
        .spawn();

    match command {
        Ok(mut child) => match child.wait() {
            Ok(status_code) => process::exit(status_code.code().unwrap_or(130)), // 128 + signal
            Err(_) => process::exit(64), // not used yet
        },
        Err(_) => process::exit(127), // command not found
    }
}
