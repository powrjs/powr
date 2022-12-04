use crate::backend_runner::{link_to_binary, link_to_obj, remove_file};
use crate::middleend_runner::run_opt;
use compiler::Compiler;
use deno_ast::{parse_script, Diagnostic, ParseParams, ParsedSource, SourceTextInfo};
use inkwell::context::Context;
use seahorse::{App, Command, Flag, FlagType};
use std::env::args;
use std::process::exit;

mod backend_runner;
mod compiler;
mod middleend_runner;

fn main() {
    let args: Vec<String> = args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .usage("powr [OPTIONS] [FILE]")
        .command(compile_command());

    app.run(args);
}

fn compile_command() -> Command {
    Command::new("compile")
        .alias("c")
        .description("Compile a TypeScript/JavaScript file")
        .usage("powr compile [FILE] [OPTIONS]")
        .flag(emmit_llvm_ir_flag())
        .flag(dry_run_flag())
        .flag(show_variables_flag())
        .action(compile_action)
}

fn emmit_llvm_ir_flag() -> Flag {
    Flag::new("emmit-llvm", FlagType::Bool)
        .description("Emmit LLVM IR")
        .alias("e")
}

fn dry_run_flag() -> Flag {
    Flag::new("dry-run", FlagType::Bool)
        .description("Only emits the LLVM IR")
        .alias("d")
}

fn show_variables_flag() -> Flag {
    Flag::new("show-variables", FlagType::Bool)
        .description("Show variables")
        .alias("s")
}

fn compile_action(ctx: &seahorse::Context) {
    let file = ctx.args.first().unwrap();
    let parsed_script = get_parsed_script(ctx);
    if parsed_script.is_err() {
        eprintln!("Failed to parse script: \n{:?}", parsed_script);
        exit(1);
    }

    let context = Context::create();
    let mut compiler = Compiler::new(&context, file);

    compiler
        .compile_main_function()
        .expect("Should be able to compile main function");

    match compiler.compile(parsed_script.unwrap().program_ref()) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }

    // TODO: use regex
    let file = file.replace(".ts", ".ll").replace(".js", ".ll");

    if ctx.bool_flag("show-variables") {
        compiler.show_variables();
    }

    let dry_run = ctx.bool_flag("dry-run");
    if !dry_run {
        match compiler.write_to_file(&file) {
            Ok(_) => println!("Compiled successfully to '{}'", file),
            Err(err) => {
                eprintln!("Failed to write to file: {}", err);
                exit(1);
            }
        }

        run_opt(&file);
    } else {
        println!("{}", compiler.get_llvm_ir());
        exit(0);
    }

    let obj_file_path = link_to_obj(&file);
    link_to_binary(&file);

    remove_file(&obj_file_path);
    if !ctx.bool_flag("emmit-llvm") {
        remove_file(&file);
    }
}

fn get_parsed_script(ctx: &seahorse::Context) -> Result<ParsedSource, Diagnostic> {
    let file = ctx.args.first().unwrap();
    if !file.ends_with(".js") && !file.ends_with(".ts") {
        eprintln!("File must be a TypeScript or JavaScript file");
        exit(1);
    }

    let code = std::fs::read_to_string(file).expect("Failed to read file");
    let text_info = SourceTextInfo::new(code.into());
    let parsed_script = parse_script(ParseParams {
        specifier: format!("file:///{}", file).into(),
        media_type: match file {
            _ if file.ends_with(".js") => deno_ast::MediaType::JavaScript,
            _ if file.ends_with(".ts") => deno_ast::MediaType::TypeScript,
            _ => unreachable!(),
        },
        text_info,
        capture_tokens: true,
        maybe_syntax: None,
        scope_analysis: true,
    });

    parsed_script
}
