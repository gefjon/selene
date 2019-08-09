use crate::lisp;
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

fn digit_to_int(c: char) -> u64 {
    match c {
        '0' ..= '9' => { (c as u64) - ('0' as u64) }
        'a' ..= 'f' => { (c as u64) - ('a' as u64) + 0xa }
        'A' ..= 'F' => { (c as u64) - ('A' as u64) + 0xa }
        _ => unreachable!(),
    }
}

fn hex_integer(s: &str) -> Result<u64> {
    let (rest, digits) = hex_digit1(s)?;
    let mut int = 0;
    for digit in digits.chars() {
        int *= 0x10;
        int += digit_to_int(digit);
    }
    Ok((rest, int))
}

fn sign(s: &str) -> Result<i64> {
    alt((value(0, char('+')),
         value(-1, char('-'))))(s)
}

fn decimal_integer(s: &str) -> Result<u64> {
    let (rest, digits) = digit1(s)?;
    let mut int = 0;
    for digit in digits.chars() {
        int *= 10;
        int += digit_to_int(digit);
    }
    Ok((rest, int))
}

fn signed_integer(s: &str) -> Result<i64> {
    map(tuple((sign, decimal_integer)),
        |(sign, int)|
        if sign == -1 { (int as i64) * -1 } else { int as i64 })(s)
}

fn integer(s: &str) -> Result<i64> {
    alt((map(preceded(tuple((char('0'), char('x'))), hex_integer),
             |i| i as i64),
         signed_integer))(s)
}

macro_rules! map_object_from {
    ($inner:path, $outer: ident) => {
        fn $outer (s: &str) -> Result<lisp::Object> {
            map($inner, From::from)(s)
        }
    }
}

map_object_from!(integer, integer_obj);
map_object_from!(symbol, symbol_obj);
map_object_from!(list, list_obj);

pub fn read(s: &str) -> Result<lisp::Object> {
    token!(alt((list_obj, integer_obj, symbol_obj)))(s)
}
