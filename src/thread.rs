use crate::{compile, err, lisp};

#[derive(Default)]
pub struct Thread {
    value_stack: Vec<lisp::Object>,
}

impl Thread {
    /// evaluate a function body and return its results.
    ///
    /// note that this will leave the `Thread` in an inconsistent
    /// state unless `function` leaves the stack with exactly one more
    /// item than was there originally. a `debug_assert` enforces this
    /// behavior.
    pub fn invoke(
        &mut self,
        function: &compile::CompiledFunction,
    ) -> err::Result<lisp::Object> {
        let _stack_count = self.value_stack.len();
        self.execute_function_body(function)?;
        debug_assert_eq!(_stack_count + 1, self.value_stack.len());
        self.pop()
    }

    fn execute_function_body(
        &mut self,
        function: &compile::CompiledFunction,
    ) -> err::Result<()> {
        for op in &**function {
            self.operate(op)?;
        }
        Ok(())
    }

    pub fn eval(&mut self, it: lisp::Object) -> err::Result<lisp::Object> {
        match it {
            lisp::Object::List(l) => {
                let f = compile::compile_function(l)?;
                self.invoke(&f)
            }
            _ => Ok(it),
        }
    }

    fn push(&mut self, value: lisp::Object) -> err::Result<()> {
        self.value_stack.push(value.shallow_copy());
        Ok(())
    }

    fn pop(&mut self) -> err::Result<lisp::Object> {
        self.value_stack.pop().ok_or(err::Error::StackUnderflow)
    }

    fn fixnum_add(&mut self) -> err::Result<()> {
        use std::convert::TryInto;
        
        let first: lisp::Fixnum = self.pop()?.try_into()?;
        let second: lisp::Fixnum = self.pop()?.try_into()?;
        let res: lisp::Fixnum = first + second;
        self.push(res.into())
    }

    fn operate(&mut self, op: &compile::Instruction) -> err::Result<()> {
        use crate::compile::Instruction::*;
        match op {
            Literal(obj) => self.push(obj.shallow_copy()),
            FixnumAdd => self.fixnum_add(),
        }
    }
}
