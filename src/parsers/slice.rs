use indexmap::IndexMap;
use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammars/slice.pest"]
pub struct SliceParser;

pub fn parse_into_indexmap(config: &String) -> Result<IndexMap<String, String>, Error<Rule>> {
    let config = SliceParser::parse(Rule::FILE, &config)?.next().unwrap();

    let mut map = IndexMap::new();
    for line in config.into_inner() {
        match line.as_rule() {
            Rule::PROPERTY => {
                let mut inner_rules = line.into_inner();
                let name = inner_rules.next().unwrap().as_str().to_string();
                let value = inner_rules.next().unwrap().as_str().to_string();
                map.insert(name, value);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(map)
}
