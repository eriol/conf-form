// Copyright © 2018 Daniele Tricoli <eriol@mornie.org>.
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[macro_use]
extern crate pest_derive;

extern crate clap;
extern crate indexmap;
extern crate pest;

pub mod parsers;

use std::collections::BTreeMap;
use std::fs;
use std::process;

use clap::{App, Arg};

use crate::parsers::slice;

const VERSION: &str = "0.1.0";
const AUTHOR: &str = "Daniele Tricoli <eriol@mornie.org>";

fn main() {
    let matches = App::new("conf-form")
        .version(VERSION)
        .author(AUTHOR)
        .about("Fill configuration files with values from profiles")
        .arg(
            Arg::with_name("config")
                .help("The config file used as template")
                .long("config")
                .required(true)
                .short("c")
                .takes_value(true)
                .value_name("FILE"),
        ).arg(
            Arg::with_name("profile")
                .help("The profile used to fill the template")
                .long("profile")
                .required(true)
                .short("p")
                .takes_value(true)
                .value_name("FILE"),
        ).get_matches();

    let config_file = fs::read_to_string(matches.value_of("config").unwrap())
        .expect("Can't read the configuration file.");

    let mut config = match slice::parse(&config_file) {
        Ok(config) => config,
        Err(err) => {
            println!("An error occurred parsing configuration file:");
            println!("{}", err);
            process::exit(1);
        }
    };

    let profile = fs::read_to_string(matches.value_of("profile").unwrap())
        .expect("Can't read the profile file.");
    let profile_map: BTreeMap<String, String> = serde_yaml::from_str(&profile).unwrap();

    config.update(profile_map);

    config.print();
}
