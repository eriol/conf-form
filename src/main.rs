#[macro_use]
extern crate pest_derive;

extern crate clap;
extern crate indexmap;
extern crate pest;

pub mod parsers;

use std::collections::BTreeMap;
use std::fs;

use clap::{App, Arg};
use indexmap::IndexMap;
use pest::Parser;

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

    let config = matches.value_of("config").unwrap();
    let unparsed_config = fs::read_to_string(config).expect(&format!("cannot read {}", config));

    let parsed_config = slice::SliceParser::parse(slice::Rule::FILE, &unparsed_config)
        .expect("Not able to parse")
        .next()
        .unwrap();

    let mut parsed_map = IndexMap::new();
    for line in parsed_config.into_inner() {
        match line.as_rule() {
            slice::Rule::PROPERTY => {
                let mut inner_rules = line.into_inner();
                let name = inner_rules.next().unwrap().as_str().to_string();
                let value = inner_rules.next().unwrap().as_str().to_string();
                parsed_map.insert(name, value);
            }
            slice::Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let profile = matches.value_of("profile").unwrap();
    let profile = fs::read_to_string(profile).expect(&format!("cannot read {}", profile));
    let deserialized_map: BTreeMap<String, String> = serde_yaml::from_str(&profile).unwrap();
    for (k, v) in deserialized_map {
        if let Some(val) = parsed_map.get_mut(&k) {
            *val = v;
        }
    }

    for (k, v) in &parsed_map {
        println!("{} = {}", k, v);
    }
}
