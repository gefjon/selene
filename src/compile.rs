use crate::{lisp::{self, fxn}, err};
use std::{convert::{TryInto, TryFrom}, ops};

#[derive(Debug)]
pub enum Instruction {
    Literal(lisp::Object),
    FixnumAdd,
}

#[derive(Debug)]
pub struct CompiledFunction {
    body: Vec<Instruction>,
}

impl ops::Deref for CompiledFunction {
    type Target = [Instruction];
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

fn special_toplevel_form_p(list: &[lisp::Object]) -> bool {
    if let Some(sym) = list.get(0)
        .map(lisp::Object::shallow_copy)
        .and_then(|o| lisp::Symbol::try_from(o).ok()) {
        match sym.name {
            "defun" => true,
            _ => false,
        }
    } else {
        false
    }
}
fn compile_special_toplevel_form(list: &lisp::List) -> err::Result<()> {
    Ok(())
}

pub fn compile_toplevel_form(form: lisp::Object) -> err::Result<Option<CompiledFunction>> {
    if let Ok(list) = lisp::List::try_from(form.shallow_copy()) {
        if special_toplevel_form_p(&list) {
            compile_special_toplevel_form(&list).map(|_| None)
        } else {
            compile_function(&[form]).map(|f| Some(f))
        }
    } else {
        Err(err::type_error())
    }
}

pub fn compile_function(body: &[lisp::Object]) -> err::Result<CompiledFunction> {
    let mut func = CompiledFunction::empty();
    for form in body {
        func.compile(form.shallow_copy())?;
    }
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
        if let Some(lisp::Object::Symbol(lisp::Symbol {
            name,
            ..
        })) = form.get(0) {
            match *name {
                "add" => self.compile_addition(&form[1..]),
                _ => Err(err::Error::UnknownCompilerForm(form)),
            }
        } else {
            Err(err::type_error())
        }
    }
    
    fn compile_addition(&mut self, args: &[lisp::Object]) -> err::Result<()> {
        for arg in args {
            self.compile(arg.shallow_copy())?;
        }
        self.append_op(Instruction::FixnumAdd);
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
        
        let compiled = compile_function(&[form.clone().into()]).expect("compiler error");
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
            vec![lisp::Symbol::intern("add").into(),
                 lisp::Object::Fixnum(fxn(3)),
                 lisp::Object::Fixnum(fxn(4))],
            fxn(7),
        );
    }
    #[test]
    fn nested_addition() {
        let plus: lisp::Object = lisp::Symbol::intern("add").into();
        let one = lisp::Object::Fixnum(fxn(1));
        let form = vec![
            plus.shallow_copy(),
            one.shallow_copy(),
            lisp::List::from(vec![
                plus,
                one.shallow_copy(),
                one,
            ]).into(),
        ];

        compile_and_execute_form(form, fxn(3));
    }
}
