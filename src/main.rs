#![feature(hash_set_entry)]

use std::io::{self, prelude::*};

mod err;
mod lisp;
mod fixnum;
mod top_type;
mod symbol;
mod list;
mod read;
mod compile;
mod thread;

fn prompt() -> io::Result<()> {
    print!("? ");
    io::stdout().flush()
}

fn main() -> err::Result<()> {
    let mut main_thread = thread::Thread::default();
    let input_buffer = &mut String::new();

    prompt()?;

    while io::stdin().read_line(input_buffer)? > 0 {
        let mut input = &input_buffer[..];
        while input.len() > 0 {
            match read::read(input) {
                Ok((rest, ast)) => {
                    input = rest;
                    match main_thread.eval(ast) {
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
