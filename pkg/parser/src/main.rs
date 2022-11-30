use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct JavaScriptParser;

fn main() {
    match JavaScriptParser::parse(Rule::program, "function sum(a, b) { return a + b; }") {
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

        print_inner(inner_pair, level + 1);
    }
}
