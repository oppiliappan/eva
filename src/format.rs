use num::{BigInt, FromPrimitive, ToPrimitive};
use std::cmp::Ordering;

use crate::error::CalcError;
use crate::CONFIGURATION;

pub fn autobalance_parens(input: &str) -> Result<String, CalcError> {
    let mut balanced = String::from(input);
    let mut left_parens = 0;
    let mut right_parens = 0;
    for letter in input.chars() {
        if letter == '(' {
            left_parens += 1;
        } else if letter == ')' {
            right_parens += 1;
        }
    }

    match left_parens.cmp(&right_parens) {
        Ordering::Greater => {
            let extras = ")".repeat(left_parens - right_parens);
            balanced.push_str(&extras[..]);
            Ok(balanced)
        }
        Ordering::Equal => Ok(balanced),
        Ordering::Less => Err(CalcError::Syntax("Mismatched parentheses!".into())),
    }
}

fn radix_fmt(number: f64, obase: usize) -> Result<String, CalcError> {
    match (number.is_infinite(), number.is_sign_positive()) {
        (true, true) => return Ok("inf".to_string()),
        (true, false) => return Ok("-inf".to_string()),
        _ => (),
    }

    if number.is_nan() {
        return Ok("nan".to_string());
    }

    let table: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    // format integral part of float
    let mut integral = BigInt::from_f64(number.abs().trunc()).unwrap();
    let mut obase_int = String::new();
    let obaseb = BigInt::from_usize(obase).unwrap();

    while integral >= obaseb {
        obase_int.push(table[(&integral % &obaseb).to_usize().unwrap()]);
        integral /= &obaseb;
    }
    obase_int.push(table[integral.to_usize().unwrap()]);
    if number.is_sign_negative() {
        obase_int.push('-');
    }
    let obase_int = obase_int.chars().rev().collect::<String>();

    // format fractional part of float
    let mut fract = number.abs().fract();
    let mut obase_fract = String::new();
    let mut i = 0;
    loop {
        fract *= obase as f64;
        obase_fract.push(table[fract.trunc() as usize]);
        i += 1;
        if fract.fract() == 0. || i >= CONFIGURATION.fix {
            break;
        }
        fract = fract.fract();
    }
    Ok(format!("{}.{}", obase_int, obase_fract))
}

fn thousand_sep(inp: &str) -> String {
    let mut result_string = String::new();
    for (i, c) in inp.to_string().chars().rev().enumerate() {
        if i % 3 == 0 && i != 0 && c.to_string() != "-" {
            result_string.push(',');
        }
        result_string.push(c)
    }
    result_string.chars().rev().collect::<String>()
}

pub fn pprint(ans: f64) {
    let ans_string = radix_fmt(ans, CONFIGURATION.base).unwrap();
    let ans_vector: Vec<&str> = ans_string.split('.').collect();
    match ans_vector.len() {
        1 => println!("{:>10}", thousand_sep(ans_vector[0])),
        2 => println!("{:>10}.{}", thousand_sep(ans_vector[0]), ans_vector[1]),
        _ => unreachable!("N-nani?!"),
    }
}
