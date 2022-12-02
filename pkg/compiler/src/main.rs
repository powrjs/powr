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

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: {} <javascript code>", args[0]);
        exit(1);
    }

    let code = &args[1];
    let code = code.as_str();

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

    let void = ctx.void_type();
    let fn_type = void.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let entry = ctx.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    println!("{}\n", code);
    let parsed = parsed.unwrap();
    #[allow(unused_variables)]
    for statement in &parsed.script().body {
        handle_statement(statement, &ctx, &module, &builder, &engine);
    }

    builder.build_return(Some(&ctx.i32_type().const_int(0, false)));

    println!("\n\n{}", &module.print_to_string().to_string());
}

fn handle_statement(
    statement: &Stmt,
    ctx: &Context,
    _module: &Module,
    builder: &Builder,
    _engine: &ExecutionEngine,
) {
    match statement {
        Expr(expr) => {
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
        Decl(decl) => {
            // assumes it's a variable declaration
            let var = decl.as_var().unwrap();
            let kind = var.kind;
            let declarations = &var.decls;

            for declaration in declarations {
                let identifier = declaration.name.as_ident().unwrap();
                let name = identifier.sym.to_string();
                let init = declaration.init.as_ref().unwrap();
                let init = init.as_lit().unwrap();
                let init = match init {
                    Lit::Num(num) => num.value,
                    _ => unreachable!("init is not a number"),
                };

                println!("{} {} = {};", kind.to_string(), name, init);

                let number_type = ctx.f64_type();
                let number = number_type.const_float(init);
                let number_ptr = builder.build_alloca(number_type, &name);
                let _ = builder.build_store(number_ptr, number);
            }
        }
        _ => {}
    }
}
