use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct CompilerError;

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred while compiling")
    }
}

impl Error for CompilerError {}

pub struct Compiler<'a, 'ctx> {
    pub context: &'a Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
    main_fn: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    #[inline]
    fn main_fn(&self) -> FunctionValue<'ctx> {
        self.main_fn.unwrap()
    }
}
