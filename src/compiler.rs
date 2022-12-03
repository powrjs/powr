use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use std::collections::HashMap;

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
