use crate::{lisp, err};
use std::ops;

pub enum Instruction {
    Literal(lisp::Object),
    IntegerAdd,
}

pub struct CompiledFunction {
    body: Vec<Instruction>,
}

impl ops::Deref for CompiledFunction {
    type Target = [Instruction];
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

pub fn compile_function(form: lisp::List) -> err::Result<CompiledFunction> {
    let mut func = CompiledFunction::empty();
    func.compile_form(form)?;
    Ok(func)
}

impl CompiledFunction {
    fn empty() -> Self {
        CompiledFunction {
            body: vec![],
        }
    }
    
    fn append_op(&mut self, op: Instruction) {
        self.body.push(op);
    }

    fn compile(&mut self, form: lisp::Object) -> err::Result<()> {
        match form {
            lisp::Object::List(form) => self.compile_form(form)?,
            _ => self.append_op(Instruction::Literal(form)),
        }
        Ok(())
    }

    fn compile_form(&mut self, form: lisp::List) -> err::Result<()> {
        match form[0] {
            lisp::Object::Symbol(lisp::Symbol {
                name: "+"
            }) => self.compile_addition(&form[1..]),
            _ => Err(err::Error::UnknownCompilerForm(form)),
        }
    }
    
    fn compile_addition(&mut self, args: &[lisp::Object]) -> err::Result<()> {
        for arg in args {
            self.compile(arg.shallow_copy())?;
        }
        self.append_op(Instruction::IntegerAdd);
        Ok(())
    }
}



#[cfg(test)]
mod test {
    use super::*;
    fn compile_and_execute_form<Form, Result>(
        form: Form,
        expected_result: Result,
    ) where
        Result: Into<lisp::Object>,
        Form: Into<lisp::List>,
    {
        let form = form.into();
        let expected_result = expected_result.into();
        
        let compiled = compile_function(form.clone()).expect("compiler error");
        let mut thread = crate::thread::Thread::default();
        let real_result = thread.invoke(&compiled).expect("evaluation error");
        if real_result != expected_result {
            panic!(
                "{:?}: expected {:?}, got {:?}",
                form,
                expected_result,
                real_result,
            );
        }
    }
    #[test]
    fn simple_addition() {
        compile_and_execute_form(
            vec![lisp::Symbol::intern("+").into(),
                 lisp::Object::Integer(3),
                 lisp::Object::Integer(4)],
            7,
        );
    }
    #[test]
    fn nested_addition() {
        let plus: lisp::Object = lisp::Symbol::intern("+").into();
        let one = lisp::Object::Integer(1);
        let form = vec![
            plus.shallow_copy(),
            one.shallow_copy(),
            lisp::List::from(vec![
                plus,
                one.shallow_copy(),
                one,
            ]).into(),
        ];

        compile_and_execute_form(form, 3);
    }
}
