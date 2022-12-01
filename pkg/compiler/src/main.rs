use deno_ast::swc::ast::Stmt;
use deno_ast::{parse_script, ParseParams, SourceTextInfo};
use std::process::exit;
#[allow(unused_imports)]
use Stmt::*;

fn main() {
    let code = r"
        let a = 1;
        let b = 2;
        console.log(a + b);
    ";
    let text_info = SourceTextInfo::new(code.into());
    let parsed = parse_script(ParseParams {
        specifier: "file:///foo/bar.ts".into(),
        media_type: deno_ast::MediaType::TypeScript,
        text_info,
        capture_tokens: true,
        maybe_syntax: None,
        scope_analysis: false,
    });

    if parsed.is_err() {
        let diag = parsed.err().unwrap();
        eprintln!("{}\n\nExiting now...", diag.message());
        exit(1);
    }

    println!("{}\n", code);
    let parsed = parsed.unwrap();
    #[allow(unused_variables)]
    for statement in &parsed.script().body {
        handle_statement(statement);
    }
}

fn handle_statement(statement: &Stmt) {
    match statement {
        rest => println!("{:#?}", rest),
    }
}
