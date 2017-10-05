// Copyright (c) 2017 László Nagy
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate clap;

use std::env;
use std::process;
use clap::{App, Arg};

mod parameters;

fn main() {
    let default_cc_compiler = env::var("CC").unwrap_or("cc".to_string());
    let default_cxx_compiler = env::var("CXX").unwrap_or("c++".to_string());

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("output")
            .long("output")
            .short("o")
            .takes_value(true)
            .value_name("file")
            .default_value("compile_commands.json")
            .help("The compilation database file"))
        .arg(Arg::with_name("cc_compiler")
            .long("use-cc")
            .takes_value(true)
            .value_name("compiler")
            .default_value(default_cc_compiler.as_str())
            .help("The C compiler which will be used in compiler wrappers"))
        .arg(Arg::with_name("cxx_compiler")
            .long("use-c++")
            .takes_value(true)
            .value_name("compiler")
            .default_value(default_cxx_compiler.as_str())
            .help("The C++ compiler which will be used in compiler wrappers"))
        .arg(Arg::with_name("relative")
            .long("relative")
            .takes_value(true)
            .value_name("directory")
            .help("Makes the database entries relative to this directory"))
        .arg(Arg::with_name("build")
            .value_name("command")
            .takes_value(true)
            .multiple(true)
            .required(true)
            .help("The build command to intercept"))
        .get_matches();

    let config = parameters::Parameters {
        cc: matches.value_of("cc_compiler").unwrap().to_string(),
        cxx: matches.value_of("cxx_compiler").unwrap().to_string(),
    };
    let (config_key, config_val) = config.to_env();
    let build: Vec<_> = matches.values_of("build").unwrap().collect();

    let mut command = process::Command::new(build[0]);
    command.args(&build[1..]);
    command.env(config_key, config_val);
    command.env("CC", "bear-cc");
    command.env("CXX", "bear-cxx");

    match command.spawn() {
        Ok(mut child) => match child.wait() {
            Ok(status_code) => process::exit(status_code.code().unwrap_or(130)), // 128 + signal
            Err(_) => process::exit(64), // not used yet
        },
        Err(_) => process::exit(127), // command not found
    }
}
