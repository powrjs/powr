use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "javascript.pest"]
struct JavaScriptParser;

#[cfg(test)]
mod tests {
    use crate::{JavaScriptParser, Rule};
    use pest::Parser;

    #[test]
    fn it_works() {
        let js = JavaScriptParser::parse(Rule::field, "123").unwrap();
    }
}
