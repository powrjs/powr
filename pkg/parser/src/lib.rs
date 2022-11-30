use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct JavaScriptParser;

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;

    fn parse(input: &str) {
        JavaScriptParser::parse(Rule::program, input).unwrap();
    }

    #[test]
    fn it_works() {
        parse("1 + 1;")
    }

    #[test]
    fn booleans() {
        parse("true;");
        parse("false;");
    }

    #[test]
    fn boolean_checking() {
        parse("true == true;");
        parse("true == false;");
        parse("false == true;");
        parse("false == false;");
    }

    #[test]
    fn scientific_notation() {
        parse("1e1;");
        parse("1e-1;");
        parse("1e+1;");
        parse("1e0;");
        parse("1e-0;");
        parse("1e+0;");
    }

    #[test]
    fn arrays() {
        parse("[];");
        parse("[1];");
        parse("[1, 2];");
        parse("[1, 2, 3];");
    }

    #[test]
    fn objects() {
        parse("{};");
        parse("{a: 1};");
        parse("{a: 1, b: 2};");
        parse("{a: 1, b: 2, c: 3};");
        parse("({a: 1, b: 2, c: 3});");
    }

    #[test]
    fn functions() {
        parse("function() {}");
        parse("function() { return 1; }");
    }
    }
}
