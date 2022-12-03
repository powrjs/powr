use compiler::Compiler;
use deno_ast::{parse_script, Diagnostic, ParseParams, ParsedSource, SourceTextInfo};
use inkwell::context::Context;
use std::process::exit;

mod compiler;

fn main() {
    let parsed_script = get_parsed_script();
    if parsed_script.is_err() {
        eprintln!("Failed to parse script: \n{:?}", parsed_script);
        exit(1);
    }

    let context = Context::create();
    let mut compiler = Compiler::new(&context);
    compiler
        .compile_main_function()
        .expect("Should be able to compile main function");
    match compiler.compile(parsed_script.unwrap().program_ref()) {
        Ok(_) => println!("Compiled successfully to './script'"), // TODO: make this configurable
        Err(err) => {
            eprintln!("Failed to compile script: {:?}", err);
            exit(1);
        }
    }
}

fn get_parsed_script() -> Result<ParsedSource, Diagnostic> {
    let code = get_code();
    let text_info = SourceTextInfo::new(code.into());
    let parsed_script = parse_script(ParseParams {
        specifier: "file:///main.ts".into(),         // FIXME
        media_type: deno_ast::MediaType::TypeScript, // FIXME
        text_info,
        capture_tokens: true,
        maybe_syntax: None,
        scope_analysis: true,
    });

    parsed_script
}

fn get_code() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage:");
        eprintln!("\t{} [compile|c] [js/ts file]", args[0]);
        eprintln!("\t{} [run|r] [js/ts code]", args[0]);
        exit(1);
    }

    let action = args[1].clone();
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

    code
}
