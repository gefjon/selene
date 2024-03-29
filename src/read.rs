use crate::lisp::{self, fxn};
use nom::{
    IResult,
    sequence::{preceded, delimited, tuple},
    branch::alt,
    multi::many0,
    character::complete::{hex_digit1, digit1, multispace0, alphanumeric1, char},
    combinator::{map, value},
};

pub type Result<'a, T> = IResult<&'a str, T>;

macro_rules! token {
    ($($args:tt)*) => {
        delimited(multispace0, $($args)*, multispace0)
    }
}

fn list(s: &str) -> Result<lisp::List> {
    map(delimited(char('('), many0(read), char(')')), From::from)(s)
}

fn symbol(s: &str) -> Result<lisp::Symbol> {
    map(alphanumeric1, lisp::Symbol::intern)(s)
}

fn digit_to_int(c: char) -> lisp::Fixnum {
    match c {
        '0' ..= '9' => { lisp::Fixnum::from(c) - lisp::Fixnum::from('0') }
        'a' ..= 'f' => { lisp::Fixnum::from(c) - lisp::Fixnum::from('a') + fxn(0xa) }
        'A' ..= 'F' => { lisp::Fixnum::from(c) - lisp::Fixnum::from('A') + fxn(0xa) }
        _ => unreachable!(),
    }
}

fn hex_fixnum(s: &str) -> Result<lisp::Fixnum> {
    let (rest, digits) = hex_digit1(s)?;
    let mut int = fxn(0);
    for digit in digits.chars() {
        int *= fxn(0x10);
        int += digit_to_int(digit);
    }
    Ok((rest, int))
}

fn sign(s: &str) -> Result<lisp::Fixnum> {
    alt((value(fxn(0), char('+')),
         value(fxn(-1), char('-'))))(s)
}

fn decimal_fixnum(s: &str) -> Result<lisp::Fixnum> {
    let (rest, digits) = digit1(s)?;
    let mut int = fxn(0);
    for digit in digits.chars() {
        int *= fxn(10);
        int += digit_to_int(digit);
    }
    Ok((rest, int))
}

fn signed_fixnum(s: &str) -> Result<lisp::Fixnum> {
    map(tuple((sign, decimal_fixnum)),
        |(sign, int)|
        if sign == fxn(-1) { int * fxn(-1) } else { int })(s)
}

fn fixnum(s: &str) -> Result<lisp::Fixnum> {
    preceded(tuple((char('0'), char('x'))), hex_fixnum)(s)
}

macro_rules! map_object_from {
    ($inner:path, $outer: ident) => {
        fn $outer (s: &str) -> Result<lisp::Object> {
            map($inner, From::from)(s)
        }
    }
}

map_object_from!(fixnum, fixnum_obj);
map_object_from!(symbol, symbol_obj);
map_object_from!(list, list_obj);

pub fn read(s: &str) -> Result<lisp::Object> {
    token!(alt((list_obj, fixnum_obj, symbol_obj)))(s)
}
