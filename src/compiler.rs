use deno_ast::swc::ast::{
    CallExpr, Callee, Expr, Lit, MemberExpr, ModuleItem, Program, Script, Stmt,
};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{AnyValueEnum, BasicValue, FunctionValue, PointerValue};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct CompilerError {
    message: String,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred while compiling")
    }
}

impl Error for CompilerError {}

pub struct Compiler<'a: 'ctx, 'ctx> {
    context: &'a Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
    main_fn: Option<FunctionValue<'ctx>>,
}

impl<'a: 'ctx, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            module: context.create_module("main"),
            builder: context.create_builder(),
            variables: HashMap::new(),
            main_fn: None,
        }
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        self.module.print_to_file(path)?;
        Ok(())
    }

    #[inline]
    fn main_fn(&self) -> FunctionValue<'ctx> {
        self.main_fn.unwrap()
    }

    fn null_pointer(&self) -> PointerValue<'ctx> {
        self.context
            .i8_type()
            .ptr_type(inkwell::AddressSpace::Generic)
            .const_null()
    }

    pub fn compile(&mut self, program: &Program) -> Result<(), CompilerError> {
        if self.main_fn.is_none() {
            return Err(CompilerError {
                message: "No main function found".to_string(),
            });
        }

        match program {
            Program::Module(module) => self.compile_module(module)?,
            Program::Script(script) => self.compile_script(script)?,
        }
        self.compile_main_function_return();

        Ok(())
    }

    fn compile_main_function_return(&self) {
        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()));
    }

    pub fn compile_main_function(&mut self) -> Result<(), CompilerError> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", fn_type, None);
        let entry = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry);
        self.main_fn = Some(main_fn);

        Ok(())
    }

    fn compile_script(&mut self, script: &Script) -> Result<(), CompilerError> {
        (&script.body)
            .into_iter()
            .try_for_each(|stmt| self.compile_statement(stmt))
    }

    fn compile_module(&mut self, module: &deno_ast::swc::ast::Module) -> Result<(), CompilerError> {
        (&module.body)
            .into_iter()
            .try_for_each(|item| self.compile_module_item(item))
    }

    fn compile_module_item(&mut self, item: &ModuleItem) -> Result<(), CompilerError> {
        match item {
            ModuleItem::ModuleDecl(_) => {
                return Err(CompilerError {
                    message: "Module declarations are not supported".to_string(),
                });
            }
            ModuleItem::Stmt(stmt) => self.compile_statement(stmt),
        }
    }

    fn compile_statement(&self, statement: &Stmt) -> Result<(), CompilerError> {
        match statement {
            Stmt::Expr(expr) => self.compile_expression(expr.expr.as_ref()).map(|_| ()),
            _ => {
                return Err(CompilerError {
                    message: "Unsupported statement".to_string(),
                });
            }
        }
    }

    fn compile_expression(&self, expression: &Expr) -> Result<AnyValueEnum, CompilerError> {
        match expression {
            Expr::Call(call) => self.compile_call_expression(call),
            Expr::Member(member) => self.compile_member_expression(member),
            Expr::Lit(lit) => self.compile_literal(lit),
            _ => {
                return Err(CompilerError {
                    message: "Unsupported expression".to_string(),
                });
            }
        }
    }

    fn compile_call_expression(
        &self,
        call_expression: &CallExpr,
    ) -> Result<AnyValueEnum, CompilerError> {
        let callee = &call_expression.callee;
        let callee = match callee {
            Callee::Expr(expr) => self.compile_expression(expr),
            _ => {
                return Err(CompilerError {
                    message: "Unsupported callee".to_string(),
                });
            }
        };

        if let Err(err) = callee {
            return Err(err);
        }
        let callee = callee.unwrap();

        match callee {
            AnyValueEnum::FunctionValue(_) => {
                let arguments = (&call_expression.args)
                    .into_iter()
                    .map(|argument| self.compile_expression(argument.expr.as_ref()))
                    .collect::<Result<Vec<_>, _>>()?;
                let argument = arguments.first().unwrap();

                match argument {
                    AnyValueEnum::PointerValue(pointer) => {
                        let bitcast = self.builder.build_bitcast(
                            pointer.as_basic_value_enum(),
                            self.context
                                .i8_type()
                                .ptr_type(inkwell::AddressSpace::Generic),
                            "console_log_argument_1",
                        );
                        self.builder.build_call(
                            callee.into_function_value(),
                            &[bitcast.into()],
                            "console_log",
                        );
                    }
                    _ => {
                        return Err(CompilerError {
                            message: "Unsupported argument".to_string(),
                        });
                    }
                }
            }
            _ => {
                return Err(CompilerError {
                    message: "Unsupported callee".to_string(),
                });
            }
        }

        // returns null pointer
        Ok(self.null_pointer().into())
    }
    fn compile_member_expression(
        &self,
        member_expression: &MemberExpr,
    ) -> Result<AnyValueEnum, CompilerError> {
        // TODO: should not be hardcoded
        // let object = self.compile_expression(member_expression.obj.as_ref())?;

        let object = member_expression.obj.as_ident().unwrap();
        let prop = &member_expression.prop;
        let prop = prop.as_ident().unwrap();

        let is_console_log = object.sym.to_string() == "console" && prop.sym.to_string() == "log";
        if is_console_log {
            let i32_type = self.context.i32_type();
            let puts_type = i32_type.fn_type(
                &[self
                    .context
                    .i8_type()
                    .ptr_type(inkwell::AddressSpace::Generic)
                    .into()],
                true,
            );
            let puts = self.module.add_function("puts", puts_type, None);

            return Ok(puts.into());
        }

        Err(CompilerError {
            message: "Unsupported expression".to_string(),
        })
    }
    fn compile_literal(&self, literal: &Lit) -> Result<AnyValueEnum, CompilerError> {
        match literal {
            Lit::Str(string) => {
                // TODO: see if we can use the string directly
                // let vector_value = self.context.const_string(string.value.as_bytes(), false);
                let string_pointer =
                    unsafe { self.builder.build_global_string(&string.value, "string") };

                Ok(string_pointer.as_pointer_value().into())
            }
            Lit::Null(_) => Ok(self.null_pointer().into()),
            _ => {
                return Err(CompilerError {
                    message: "Unsupported literal".to_string(),
                });
            }
        }
    }
}
