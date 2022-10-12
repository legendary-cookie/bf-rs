use inkwell::builder::Builder;
use inkwell::context::Context;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {}
