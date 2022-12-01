use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;

fn main() {
    let ctx = Context::create();
    let module = ctx.create_module("addition");
    let number_type = ctx.f64_type();
    let sum_fn_type = number_type.fn_type(&[number_type.into(), number_type.into()], false);
    let sum_fn = module.add_function("sum", sum_fn_type, None);
    let entry = ctx.append_basic_block(sum_fn, "entry");
    let builder = ctx.create_builder();
    builder.position_at_end(entry);

    let a = sum_fn.get_nth_param(0).unwrap().into_float_value();
    let b = sum_fn.get_nth_param(1).unwrap().into_float_value();
    let ret = builder.build_float_add(a, b, "sum");
    let _return_instruction = builder.build_return(Some(&ret));

    println!("{}", module.print_to_string().to_string());

    let execution_engine = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    unsafe {
        type Addition = unsafe extern "C" fn(f64, f64) -> f64;
        let add: JitFunction<Addition> = execution_engine.get_function("sum").unwrap();
	    let (a, b) = (1.0, 2.0);
        let result = add.call(a, b);
        assert_eq!(result, a + b);
    }
}
