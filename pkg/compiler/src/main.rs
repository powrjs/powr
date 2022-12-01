use deno_ast::swc::ast::{BinaryOp, Lit, Stmt};
use deno_ast::{parse_script, ParseParams, SourceTextInfo};
use execution_engine::ExecutionEngine;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine;
use inkwell::module::Module;
use inkwell::values::AnyValue;
use std::process::exit;
#[allow(unused_imports)]
use Stmt::*;

fn main() {
    let ctx = Context::create();
    let module_name = "compiled";
    let module = ctx.create_module(module_name);
    let builder = ctx.create_builder();
    let engine = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let code = r"
        10 + 30
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
        handle_statement(statement, &ctx, &module, &builder, &engine);
    }

    println!("\n\n{}", &module.print_to_string().to_string());
}

fn handle_statement(
    statement: &Stmt,
    ctx: &Context,
    module: &Module,
    builder: &Builder,
    engine: &ExecutionEngine,
) {
    match statement {
        Expr(expr) => {
            let expr = expr.clone();
            let expr = &expr.expr;

            // assumes it's a binary expression
            let bin_expr = expr.as_bin().unwrap();
            let lhs = bin_expr.left.as_lit().unwrap();
            let rhs = bin_expr.right.as_lit().unwrap();

            let bin_op = bin_expr.op;

            let lhs = match lhs {
                Lit::Num(num) => num.value,
                _ => unreachable!("lhs is not a number"),
            };
            let rhs = match rhs {
                Lit::Num(num) => num.value,
                _ => unreachable!("rhs is not a number"),
            };

            match bin_op {
                BinaryOp::Add => {
                    let lhs = ctx.f64_type().const_float(lhs);
                    let rhs = ctx.f64_type().const_float(rhs);
                    let result = builder.build_float_add(lhs, rhs, "add_result");

                    println!("Result: {}", result.print_to_string().to_string());
                    println!("JS Value: {}", result.get_constant().unwrap().0);
                }
                _ => unreachable!("only addition is supported"),
            }
        }
        _ => {}
    }
}
