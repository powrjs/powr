use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct JavaScriptParser;

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;

    #[test]
    fn it_works() {
        JavaScriptParser::parse(Rule::program, "1 + 1;").unwrap();
    }

    #[test]
    fn booleans() {
        JavaScriptParser::parse(Rule::program, "true;").unwrap();
        JavaScriptParser::parse(Rule::program, "false;").unwrap();
    }

    #[test]
    fn boolean_checking() {
        JavaScriptParser::parse(Rule::program, "true === true;").unwrap();
        JavaScriptParser::parse(Rule::program, "true === false;").unwrap();
        JavaScriptParser::parse(Rule::program, "false === true;").unwrap();
        JavaScriptParser::parse(Rule::program, "false === false;").unwrap();
    }

    #[test]
    fn scientific_notation() {
        JavaScriptParser::parse(Rule::program, "1e-1;").unwrap();
    }

    #[test]
    fn arrays() {
        JavaScriptParser::parse(Rule::program, "[1, 2, 3];").unwrap();
    }

    #[test]
    fn objects() {
        JavaScriptParser::parse(Rule::program, "({a: 1, b: 2, c: 3});").unwrap();
    }
}
