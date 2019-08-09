#![feature(hash_set_entry)]

use std::io::{self, prelude::*};

mod lisp;
mod top_type;
mod symbol;
mod list;
mod read;
mod compile;

mod err {
    #[derive(Debug)]
    pub enum Error {
        Io(std::io::Error),
    }

    impl From<std::io::Error> for Error {
        fn from(e: std::io::Error) -> Error {
            Error::Io(e)
        }
    }
    
    pub type Result<T> = std::result::Result<T, Error>;
}

fn prompt() -> io::Result<()> {
    print!("? ");
    io::stdout().flush()
}

fn main() -> err::Result<()> {
    let input_buffer = &mut String::new();

    prompt()?;

    while io::stdin().read_line(input_buffer)? > 0 {
        let mut input = &input_buffer[..];
        while input.len() > 0 {
            match read::read(input) {
                Ok((rest, ast)) => {
                    input = rest;
                    match eval(ast) {
                        Ok(result) => println!("{}", result),
                        Err(eval_error) => eprintln!("eval_error: {:?}", eval_error),
                    }
                }
                Err(read_error) => {
                    eprintln!("read_error: {:?}", read_error);
                    input = "";
                },
            }
        }
        input_buffer.clear();
        prompt()?;
    }
    Ok(())
}

fn eval(it: lisp::Object) -> err::Result<lisp::Object> {
    match it {
        lisp::Object::List(l) => {
            let f = compile::compile_form(l)?;
            f.invoke()
        }
        _ => Ok(it),
    }
}
