use pest::Parser;
use pest_derive::Parser;
use std::fs::read;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct JavaScriptParser;

fn main() {
    let code = String::from_utf8(read("examples/values.js").unwrap()).unwrap();
    match JavaScriptParser::parse(Rule::program, &code) {
        Ok(js) => println!("{:#?}", js),
        Err(e) => eprintln!("{}", e),
    };
}
