use pest::Parser;
use pest_derive::Parser;
use process::exit;
use std::fs::read_to_string;
use std::{env, process};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct JavaScriptParser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        usage(&args[0]);
    }

    match args[1].as_str() {
        "str" | "s" => parse_code(&args[2]),
        "file" | "f" => parse_file(&args[2]),
        _ => usage(&args[0]),
    }
}

fn usage(program: &str) {
    println!("Usage:");
    println!("\t{} [str|s] [javascript code]", program);
    println!("\t{} [file|f] [javascript file]", program);
    exit(1);
}

fn parse_file(file_path: &str) {
    let file = read_to_string(file_path).unwrap();
    parse_code(&file);
}

fn parse_code(code: &str) {
    match JavaScriptParser::parse(Rule::program, code) {
        Ok(pairs) => {
            for pair in pairs {
                println!("Rule:\t{:?}", pair.as_rule());
                let text = pair
                    .as_str()
                    .lines()
                    .map(|s| format!("\t{}\n", s))
                    .collect::<String>();
                println!("Text: {}", text);
                println!("Guide:\t[rule]: string text");
                println!();

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
