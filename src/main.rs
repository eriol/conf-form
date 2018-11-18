#[macro_use]
extern crate pest_derive;

extern crate clap;
extern crate indexmap;
extern crate pest;

pub mod parsers;

use std::collections::BTreeMap;
use std::fs;

use clap::{App, Arg};

use crate::parsers::slice;

fn main() {
    let matches = App::new("conf-form")
        .version("0.1")
        .author("Daniele Tricoli <eriol@mornie.org>")
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

    let config = fs::read_to_string(matches.value_of("config").unwrap())
        .expect("Can't read the configuration file.");
    let mut parsed = slice::parse(&config).expect("Unable to parse.");

    let profile = fs::read_to_string(matches.value_of("profile").unwrap())
        .expect("Can't read the profile file.");
    let profile_map: BTreeMap<String, String> = serde_yaml::from_str(&profile).unwrap();

    parsed.update(profile_map);

    parsed.print();
}
