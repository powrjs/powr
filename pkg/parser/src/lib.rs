use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct JavaScriptParser;

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::{consumes_to, parses_to, Parser};

    fn parse(input: &str) {
        JavaScriptParser::parse(Rule::program, input).unwrap();
    }

    fn parse_rule(input: &str, rule: Rule) {
        JavaScriptParser::parse(rule, input).unwrap();
    }

    #[test]
    fn it_works() {
        parses_to! {
            parser: JavaScriptParser,
            input: "1 + 1;",
            rule: Rule::program,
            tokens: [
                program(0, 6, [
                    literal(0, 1, [
                        number(0, 1)
                    ]),
                    operator(2, 3),
                    literal(4, 5, [
                        number(4, 5)
                    ]),
                    EOI(6, 6)
                ])
            ]
        }
    }

    #[test]
    fn boolean_checking() {
        parses_to! {
            parser: JavaScriptParser,
            input: "true == true;",
            rule: Rule::program,
            tokens: [
                program(0, 13, [
                    literal(0, 4, [
                        boolean(0, 4)
                    ]),
                    operator(5, 7),
                    literal(8, 12, [
                        boolean(8, 12)
                    ]),
                    EOI(13, 13)
                ])
            ]
        }

        parses_to! {
            parser: JavaScriptParser,
            input: "true == false;",
            rule: Rule::program,
            tokens: [
                program(0, 14, [
                    literal(0, 4, [
                        boolean(0, 4)
                    ]),
                    operator(5, 7),
                    literal(8, 13, [
                        boolean(8, 13)
                    ]),
                    EOI(14, 14)
                ])
            ]
        }
    }

    #[test]
    fn scientific_notation() {
        parses_to! {
            parser: JavaScriptParser,
            input: "1e1;",
            rule: Rule::program,
            tokens: [
                program(0, 4, [
                    literal(0, 3, [
                        number(0, 3)
                    ]),
                    EOI(4, 4)
                ])
            ]
        }

        parses_to! {
            parser: JavaScriptParser,
            input: "1e-1;",
            rule: Rule::program,
            tokens: [
                program(0, 5, [
                    literal(0, 4, [
                        number(0, 4)
                    ]),
                    EOI(5, 5)
                ])
            ]
        }
    }

    #[test]
    fn arrays() {
        parses_to! {
            parser: JavaScriptParser,
            input: "[];",
            rule: Rule::program,
            tokens: [
                program(0, 3, [
                    array(0, 2, []),
                    EOI(3, 3)
                ])
            ]
        }

        parses_to! {
            parser: JavaScriptParser,
            input: "[1, 2, 3];",
            rule: Rule::program,
            tokens: [
                program(0, 10, [
                    array(0, 9, [
                        literal(1, 2, [
                            number(1, 2)
                        ]),
                        literal(4, 5, [
                            number(4, 5)
                        ]),
                        literal(7, 8, [
                            number(7, 8)
                        ])
                    ]),
                    EOI(10, 10)
                ])
            ]
        }
    }

    #[test]
    fn objects() {
        parse("{};");
        parse("{a: 1};");
        parse("{a: 1, b: 2};");
        parse("{a: 1, b: 2, c: 3};");
        parse("({a: 1, b: 2, c: 3});");

        parses_to! {
            parser: JavaScriptParser,
            input: "{a: 1, b: 2, c: 3};",
            rule: Rule::program,
            tokens: [
                program(0, 20, [
                    object(0, 19, [
                        pair(1, 6, [
                            identifier(1, 2),
                            literal(5, 6, [
                                number(5, 6)
                            ])
                        ]),
                        pair(8, 13, [
                            identifier(8, 9),
                            literal(12, 13, [
                                number(12, 13)
                            ])
                        ]),
                        pair(15, 20, [
                            identifier(15, 16),
                            literal(19, 20, [
                                number(19, 20)
                            ])
                        ])
                    ]),
                    EOI(20, 20)
                ])
            ]
        }
    }

    #[test]
    fn functions() {
        parse("function() {}");
        parse("function() { return 1; }");
        parse("function sum(a, b) { return a + b; }");
    }

    #[test]
    fn assignments() {
        parse("const a;");
        parse("const a = b;");
        parse("let c = d;");
        parse("var e = f;");
        parse("foo = bar");
        parse("foo += bar");
        parse("foo -= bar");
        parse("foo *= bar");
        parse("foo /= bar");
        parse("foo %= bar");
        parse("foo **= bar");
        parse("foo <<= bar");
        parse("foo >>= bar");
        parse("foo >>>= bar");
        parse("foo &= bar");
        parse("foo ^= bar");
        parse("foo |= bar");
    }

    #[test]
    fn operators() {
        parse_rule("1 + 1", Rule::expression);
        parse_rule("1 - 1", Rule::expression);
        parse_rule("1 * 1", Rule::expression);
        parse_rule("1 / 1;", Rule::expression);
        parse_rule("1 % 1;", Rule::expression);
        parse_rule("1 ** 1;", Rule::expression);
        parse_rule("1 << 1;", Rule::expression);
        parse_rule("1 >> 1;", Rule::expression);
        parse_rule("1 >>> 1;", Rule::expression);
        parse_rule("1 & 1;", Rule::expression);
        parse_rule("1 | 1;", Rule::expression);
        parse_rule("1 ^ 1;", Rule::expression);
        parse_rule("1 && 1;", Rule::expression);
        parse_rule("1 || 1;", Rule::expression);
        parse_rule("1 ?? 1;", Rule::expression);
        parse_rule("1 == 1;", Rule::expression);
        parse_rule("1 === 1;", Rule::expression);
        parse_rule("1 != 1;", Rule::expression);
        parse_rule("1 !== 1;", Rule::expression);
        parse_rule("1 < 1;", Rule::expression);
        parse_rule("1 <= 1;", Rule::expression);
        parse_rule("1 > 1;", Rule::expression);
        parse_rule("1 >= 1;", Rule::expression);
    }

    #[test]
    fn new_lines() {
        parse("function append(a, b) {\nreturn a + b;\n}");
    }

    #[test]
    fn arrow_function() {
        parse("const sum = (a, b) => a + b;");
        parse("const sum = (a, b) => {\nreturn a + b;\n}");
    }

    #[test]
    fn if_statements() {
        parse("if (true) { return 1; }");
        parse("if (false) { return 1; }");
        parse("if (true) { return 1; } else { return 2; }");
        parse("if (true) { return 1; } else if (true) { return 3; } else { return 2; }");
    }

    #[test]
    fn do_statement() {
        parse("while (true) { return 1; }");
    }

    #[test]
    fn for_statement() {
        parse("for (let i = 0; i < 10; i++) { return 1; }");
    }
}
