use deno_ast::swc::ast::{BinaryOp, Lit, Stmt};
use deno_ast::{parse_script, ParseParams, SourceTextInfo};
use execution_engine::ExecutionEngine;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use std::process::exit;
#[allow(unused_imports)]
use Stmt::*;

fn main() {
    let ctx = Context::create();
    let module_name = "compiled";
    let module = ctx.create_module(module_name);
    let builder = ctx.create_builder();
    let engine = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::Aggressive)
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

    let i32_type = ctx.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let entry = ctx.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    let i32_type = ctx.i32_type();
    let printf_type = i32_type.fn_type(
        &[ctx
            .i8_type()
            .ptr_type(inkwell::AddressSpace::Generic)
            .into()],
        true,
    );
    let printf = module.add_function("printf", printf_type, None);

    let parsed = parsed.unwrap();
    #[allow(unused_variables)]
    for statement in &parsed.script().body {
        handle_statement(statement, &ctx, &module, &builder, &engine, printf);
    }

    builder.build_return(Some(&ctx.i32_type().const_int(0, false)));

    module.print_to_file("compiled.ll").unwrap();
}

fn handle_statement(
    statement: &Stmt,
    ctx: &Context,
    _module: &Module,
    builder: &Builder,
    _engine: &ExecutionEngine,
    printf: FunctionValue,
) {
    match statement {
        Expr(expr) => {
            let expr = &expr.expr;

            if let Some(call) = expr.as_call() {
                let callee = &call.callee;
                let callee = (&callee).as_expr().unwrap();
                let callee = callee.as_member().unwrap();
                let console = callee.obj.as_ident().unwrap();
                let log = callee.prop.as_ident().unwrap();

                let console = console.sym.to_string();
                let log = log.sym.to_string();

                if console == "console".to_string() && log == "log".to_string() {
                    let args = &call.args;

                    for arg in args {
                        let arg = arg.expr.as_lit().unwrap();
                        let arg = match arg {
                            Lit::Str(string) => string,
                            _ => unimplemented!("Only string literals are supported"),
                        };

                        let arg = format!("{}\n", arg.value);
                        let message = unsafe { builder.build_global_string(&arg, "consolelog") };
                        let message_bit_cast = builder.build_bitcast(
                            message,
                            ctx.i8_type().ptr_type(inkwell::AddressSpace::Generic),
                            "message_bit_cast",
                        );

                        builder.build_call(printf, &[message_bit_cast.into()], "callprintf");
                    }
                }

                return;
            }

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
                    let _result = builder.build_float_add(lhs, rhs, "add_result");
                }
                _ => unimplemented!("only addition is supported"),
            }
        }
        Decl(decl) => {
            // assumes it's a variable declaration
            let var = decl.as_var().unwrap();
            let _kind = var.kind;
            let declarations = &var.decls;

            for declaration in declarations {
                let identifier = declaration.name.as_ident().unwrap();
                let name = identifier.sym.to_string();
                let init = declaration.init.as_ref().unwrap();
                let init = init.as_lit().unwrap();
                let init = match init {
                    Lit::Num(num) => num.value,
                    _ => unimplemented!("init is not a number"),
                };

                let number_type = ctx.f64_type();
                let number = number_type.const_float(init);
                let number_ptr = builder.build_alloca(number_type, &name);
                let _ = builder.build_store(number_ptr, number);
            }
        }
        _ => {}
    }
}
