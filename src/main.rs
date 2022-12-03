use compiler::Compiler;
use deno_ast::{parse_script, ParseParams, SourceTextInfo};
use inkwell::context::Context;
use std::process::exit;

mod compiler;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        println!(
            "Usage:\n\t{} [compile|c] [js/ts file]\n\t{} [run|r] [js/ts code]",
            args[0], args[0]
        );
        exit(1);
    }

    let action = &args[1];
    let code = match action.as_str() {
        "compile" | "c" => {
            let file = &args[2];
            let code = std::fs::read_to_string(file).unwrap();
            code
        }
        "run" | "r" => {
            let code = &args[2];
            code.to_string()
        }
        _ => {
            eprintln!("Invalid action: {}", action);
            exit(1);
        }
    };

    let text_info = SourceTextInfo::new(code.into());
    let parsed_script = parse_script(ParseParams {
        specifier: "file:///main.ts".into(),         // FIXME
        media_type: deno_ast::MediaType::TypeScript, // FIXME
        text_info,
        capture_tokens: true,
        maybe_syntax: None,
        scope_analysis: true,
    });

    if parsed_script.is_err() {
        eprintln!("Failed to parse script: {:?}", parsed_script);
        exit(1);
    }

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let mut compiler = Compiler::new(&context, &module, &builder);
    match compiler.compile(parsed_script.unwrap().program_ref()) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to compile script: {:?}", err);
            exit(1);
        }
    }
}
