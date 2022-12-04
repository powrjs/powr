use deno_ast::swc::ast::{
    CallExpr, Callee, Decl, Expr, Lit, MemberExpr, ModuleItem, Pat, Program, Script, Stmt, VarDecl,
};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::AnyTypeEnum;
use inkwell::values::{AnyValue, AnyValueEnum, BasicValue, FunctionValue, PointerValue};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;

#[derive(Debug)]
pub struct CompilerError {
    message: String,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred while compiling: '{}'", self.message)
    }
}

impl Error for CompilerError {}

enum VariableKind {
    Const,
    Let,
    Var,
}

impl VariableKind {
    fn from_string(string: String) -> Self {
        match string.as_str() {
            "const" => VariableKind::Const,
            "let" => VariableKind::Let,
            "var" => VariableKind::Var,
            _ => panic!("Invalid variable kind"),
        }
    }
}

struct Variable<'ctx> {
    pointer: PointerValue<'ctx>,
    kind: VariableKind,
}

pub(crate) struct Compiler<'a, 'ctx> {
    context: &'a Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    variables: RefCell<HashMap<String, Variable<'ctx>>>,
    main_fn: Option<FunctionValue<'ctx>>,
}

impl<'a: 'ctx, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(context: &'a Context, source_path: &str) -> Self {
        let file = Path::new(source_path);
        let filename = file.file_name().unwrap().to_str().unwrap();

        Self {
            context,
            module: context.create_module(filename),
            builder: context.create_builder(),
            variables: RefCell::new(HashMap::new()),
            main_fn: None,
        }
    }

    pub fn get_llvm_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }

    fn add_variable(&self, name: String, pointer: PointerValue<'ctx>, kind: VariableKind) {
        self.variables
            .borrow_mut()
            .insert(name, Variable { pointer, kind });
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
            Stmt::Decl(decl) => self.compile_declaration(decl),
            _ => {
                return Err(CompilerError {
                    message: "Unsupported statement".to_string(),
                });
            }
        }
    }

    fn compile_expression(&self, expression: &Expr) -> Result<AnyValueEnum<'ctx>, CompilerError> {
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
    ) -> Result<AnyValueEnum<'ctx>, CompilerError> {
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
    ) -> Result<AnyValueEnum<'ctx>, CompilerError> {
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
    fn compile_literal(&self, literal: &Lit) -> Result<AnyValueEnum<'ctx>, CompilerError> {
        match literal {
            Lit::Str(string) => {
                // TODO: see if we can use the string directly
                // let vector_value = self.context.const_string(string.value.as_bytes(), false);
                let string_pointer =
                    unsafe { self.builder.build_global_string(&string.value, "string") };

                Ok(string_pointer.as_any_value_enum())
            }
            Lit::Num(number) => {
                let number_value = number.value;
                let f64_type = self.context.f64_type();
                let const_float = f64_type.const_float(number_value);

                Ok(const_float.as_any_value_enum())
            }
            Lit::Bool(boolean) => {
                let bool_type = self.context.bool_type();
                let const_bool = bool_type.const_int(boolean.value.into(), false);

                Ok(const_bool.as_any_value_enum())
            }
            Lit::Null(_) => Ok(self.null_pointer().into()),
            _ => {
                return Err(CompilerError {
                    message: "Unsupported literal".to_string(),
                });
            }
        }
    }

    fn compile_declaration(&self, declaration: &Decl) -> Result<(), CompilerError> {
        match declaration {
            Decl::Var(var) => self.compile_variable_declaration(var),
            _ => {
                return Err(CompilerError {
                    message: "Unsupported declaration".to_string(),
                });
            }
        }
    }
    fn compile_variable_declaration(
        &self,
        variable_declaration: &Box<VarDecl>,
    ) -> Result<(), CompilerError> {
        let declarations = &variable_declaration.decls;
        let kind = &variable_declaration.kind;

        for declaration in declarations {
            let name = self.compile_pattern(&declaration.name)?;
            let init = self.compile_expression(&declaration.init.as_ref().unwrap())?;
            let kind = VariableKind::from_string(kind.to_string());

            let pointer = self.allocate_variable(&name, &init, &kind)?;
            self.add_variable(name, pointer, kind);
        }

        Ok(())
    }

    fn allocate_variable(
        &self,
        variable_name: &String,
        init_value: &AnyValueEnum<'ctx>,
        _variable_kind: &VariableKind,
    ) -> Result<PointerValue<'ctx>, CompilerError> {
        let variable_type = init_value.get_type();

        let variable_value = match variable_type {
            AnyTypeEnum::ArrayType(array) => {
                let pointer = self.builder.build_alloca(array, &variable_name);
                self.builder
                    .build_store(pointer, init_value.into_array_value());

                pointer
            }
            AnyTypeEnum::FloatType(float) => {
                let pointer = self.builder.build_alloca(float, &variable_name);
                self.builder
                    .build_store(pointer, init_value.into_float_value());

                pointer
            }
            AnyTypeEnum::IntType(int) => {
                let pointer = self.builder.build_alloca(int, &variable_name);
                self.builder
                    .build_store(pointer, init_value.into_int_value());

                pointer
            }
            AnyTypeEnum::PointerType(pointer) => {
                let pointer = self.builder.build_alloca(pointer, &variable_name);
                self.builder
                    .build_store(pointer, init_value.into_pointer_value());

                pointer
            }
            AnyTypeEnum::StructType(structt) => {
                let pointer = self.builder.build_alloca(structt, &variable_name);
                self.builder
                    .build_store(pointer, init_value.into_struct_value());

                pointer
            }
            AnyTypeEnum::VectorType(vector) => {
                let pointer = self.builder.build_alloca(vector, &variable_name);
                self.builder
                    .build_store(pointer, init_value.into_vector_value());

                pointer
            }
            _ => {
                return Err(CompilerError {
                    message: "Unsupported variable type".to_string(),
                });
            }
        };

        Ok(variable_value)
    }

    fn compile_pattern(&self, pattern: &Pat) -> Result<String, CompilerError> {
        match pattern {
            Pat::Ident(ident) => {
                let name = ident.sym.to_string();
                Ok(name)
            }
            _ => {
                return Err(CompilerError {
                    message: "Unsupported pattern".to_string(),
                });
            }
        }
    }
}
