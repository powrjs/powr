use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "javascript.pest"]
struct JavaScriptParser;

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;

    #[test]
    fn it_works() {
        JavaScriptParser::parse(Rule::program, "if (a > b) { return 3 }").unwrap();
    }
}
