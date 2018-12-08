// Copyright © 2018 Daniele Tricoli <eriol@mornie.org>.
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[macro_use]
extern crate pest_derive;

mod parsers;

use std::collections::BTreeMap;
use std::fs;
use std::process;

use clap::{crate_authors, crate_version, App, Arg};
use colored::Colorize;

use crate::parsers::slice;

const ERROR_PARSING: &str = "An error occurred while parsing:";

fn main() {
    let matches = App::new("conf-form")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Fill configuration files with values from profiles")
        .arg(
            Arg::with_name("config")
                .help("The config file used as template")
                .long("config")
                .required(true)
                .short("c")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("profile")
                .help("The profile used to fill the template")
                .long("profile")
                .required(true)
                .short("p")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(Arg::with_name("warning").help("Show warnings").short("w"))
        .get_matches();

    // We can unwrap because config is required.
    let config_file = matches.value_of("config").unwrap();
    let config = read_file(config_file);

    let mut config = match slice::parse(&config) {
        Ok(config) => config,
        Err(err) => {
            println!("{}: {}:\n{}", ERROR_PARSING.red().bold(), &config_file, err);
            process::exit(1);
        }
    };

    // We can unwrap because profile is required.
    let profile_file = matches.value_of("profile").unwrap();
    let profile = read_file(profile_file);

    let profile: BTreeMap<String, String> = match serde_yaml::from_str(&profile) {
        Ok(profile) => profile,
        Err(err) => {
            println!("{}: {}: {}", ERROR_PARSING.red().bold(), &profile_file, err);
            process::exit(1);
        }
    };

    config.update(profile);

    config.print(matches.is_present("warning"));
}

// Read the content of a file and return it as String.
// In case of errors exit the process with return code 1.
fn read_file(f: &str) -> String {
    match fs::read_to_string(&f) {
        Ok(file) => file,
        Err(err) => {
            println!(
                "{} {}: {}",
                "An error occurred while reading:".red().bold(),
                &f,
                err
            );
            process::exit(1);
        }
    }
}
