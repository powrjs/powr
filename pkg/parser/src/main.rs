use pest::Parser;
use pest_derive::Parser;
use process::exit;
use std::{env, process};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct JavaScriptParser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} [javascript code]", args[0]);
        exit(1);
    }

    println!("{:#?}\n", JavaScriptParser::parse(Rule::program, &args[1]));
    parse_code(&args[1]);
}

fn parse_code(code: &str) {
    match JavaScriptParser::parse(Rule::program, code) {
        Ok(pairs) => {
            for pair in pairs {
                println!("Rule: {:?}", pair.as_rule());
                println!("Text: {}", pair.as_str());
                println!();
                println!("\t[rule]: string text");

                print_inner(pair, 1);
            }
        }
        Err(e) => eprintln!("{}", e),
    };
}

fn print_inner(pair: pest::iterators::Pair<Rule>, level: usize) {
    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::EOI {
            continue;
        }

        let ident = "-".repeat(level);
        println!(
            "{}> [{:?}]: {}",
            ident,
            inner_pair.as_rule(),
            inner_pair.as_str()
        );

        if inner_pair.as_rule().ne(&Rule::literal) {
            print_inner(inner_pair, level + 1);
        }
    }
}
