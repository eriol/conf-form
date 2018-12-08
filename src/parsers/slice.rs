// Copyright © 2016-2018 Daniele Tricoli <eriol@mornie.org>.
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::collections::BTreeMap;
use std::vec::Vec;

use colored::Colorize;
use indexmap::IndexMap;
use pest::error::Error;
use pest::Parser;

const SEPARATOR: &str = "=";

#[derive(Parser)]
#[grammar = "grammars/slice.pest"]
struct SliceParser;

/// zeroc configuration file.
pub struct Config {
    map: IndexMap<String, String>,
    warnings: Vec<String>,
}

/// Create Config object from a string.
pub fn parse(s: &String) -> Result<Config, Error<Rule>> {
    let parsed = SliceParser::parse(Rule::FILE, &s)?.next().unwrap();

    let mut config = Config {
        map: IndexMap::new(),
        warnings: Vec::new(),
    };

    for line in parsed.into_inner() {
        match line.as_rule() {
            Rule::PROPERTY => {
                let mut inner_rules = line.into_inner();
                let name = inner_rules.next().unwrap().as_str().to_string();
                let value = inner_rules.next().unwrap().as_str().to_string();
                config.map.insert(name, value);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(config)
}

impl Config {
    /// Overwrite Config keys using the ones from map.
    ///
    /// If a key is only in the profile, it will not be added to the
    /// configuration file, and a warning will be saved.
    pub fn update(&mut self, map: BTreeMap<String, String>) {
        for (k, v) in map {
            if let Some(val) = self.map.get_mut(&k) {
                *val = v;
            } else {
                self.warnings.push(format!(
                    "{}: {} key is not present in config file.",
                    "Warning".yellow(),
                    k
                ));
            }
        }
    }

    /// Print Config to stdout.
    ///
    /// If show_warnings is true, warnings will be printed to stderr.
    pub fn print(&self, show_warnings: bool) {
        if show_warnings {
            for warning in &self.warnings {
                eprintln!("{}", warning);
            }
        }
        for (k, v) in &self.map {
            println!("{} {} {}", k, SEPARATOR, v);
        }
    }
}

#[test]
fn parse_from_string() {
    let conf = "
        Author.name = eriol
        Author.like = rust, python
    "
    .to_string();

    let c = parse(&conf).unwrap();

    assert_eq!(c.map["Author.name"], "eriol");
    assert_eq!(c.map["Author.like"], "rust, python");
}

#[test]
fn update() {
    let conf = "
        Author.name = eriol
        Author.like = rust, python
        Author.web = example.org
    "
    .to_string();

    let mut profile = BTreeMap::new();
    profile.insert("Author.like".to_string(), "rust, python, c++".to_string());
    profile.insert("Author.web".to_string(), "example.org:8080".to_string());
    profile.insert("Only.here".to_string(), "value".to_string());

    let mut c = parse(&conf).unwrap();
    c.update(profile);

    assert_eq!(c.map["Author.like"], "rust, python, c++");
    assert_eq!(c.map["Author.web"], "example.org:8080");

    assert_eq!(c.warnings.len(), 1);
    let m: Vec<&str> = c.warnings.first().unwrap().split(":").collect();
    assert_eq!(m[1].trim(), "Only.here key is not present in config file.");
}
