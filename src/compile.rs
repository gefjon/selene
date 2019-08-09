use crate::{lisp, err};

enum Instruction {}

pub struct CompiledFunction {
    body: Vec<Instruction>,
}

impl CompiledFunction {
    pub fn invoke(&self) -> err::Result<lisp::Object> {
        unimplemented!()
    }
}

pub fn compile_form(form: lisp::List) -> err::Result<CompiledFunction> {
    unimplemented!()
}
