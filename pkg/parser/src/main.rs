use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct JavaScriptParser;

fn main() {
    match JavaScriptParser::parse(Rule::expression, "1 ** 1") {
        Ok(pairs) => {
            for pair in pairs {
                println!("Rule: {:?}", pair.as_rule());
                println!("Text: {}", pair.as_str());

                for inner_pair in pair.into_inner() {
                    if inner_pair.as_rule() == Rule::EOI {
                        continue;
                    }

                    println!("Inner Rule: {:?}", inner_pair.as_rule());
                    println!("Inner Text: {}", inner_pair.as_str());
                }
            }
        }
        Err(e) => eprintln!("{}", e),
    };
}
