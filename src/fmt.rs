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

fn thousand_sep(mut s: String) -> String {
    let inc = 3;
    let mut end = s.find('.').unwrap();
    let sign = if s.starts_with('-') { 1 } else { 0 };
    for _ in 0..(end - sign - 1) / inc {
        end -= inc;
        s.insert(end, ',');
    }
    s
}

pub fn pprint(mut ans: f64) {
    if ans.is_infinite() {
        println!("{}inf", if ans.is_sign_positive() { "" } else { "-" });
    } else if ans.is_nan() {
        println!("nan");
    } else if CONFIGURATION.base == 10 {
        // use standard library formatter since it handle printing pretty well
        let ans = format!("{:.*}", CONFIGURATION.fix, ans);
        println!("{}", thousand_sep(ans));
    } else {
        ans = format!("{:.*}", CONFIGURATION.fix, ans).parse().unwrap();
        let table: &[u8] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();

        // format integral part of float
        let mut integral = BigInt::from_f64(ans.abs().trunc()).unwrap();
        let mut obase_int = String::new();
        let obaseb = BigInt::from_usize(CONFIGURATION.base as usize).unwrap();

        while integral >= obaseb {
            obase_int.push(table[(&integral % &obaseb).to_usize().unwrap()] as char);
            integral /= &obaseb;
        }
        obase_int.push(table[integral.to_usize().unwrap()] as char);
        if ans.is_sign_negative() {
            obase_int.push('-');
        }
        let obase_int = obase_int.chars().rev().collect::<String>();

        // format fractional part of float
        // TODO: fix conversion with fractional part
        let mut fract = ans.abs().fract();
        let mut obase_fract = String::new();
        let mut i = 0;
        loop {
            fract *= CONFIGURATION.base as f64;
            obase_fract.push(table[fract.trunc() as usize] as char);
            i += 1;
            if fract.fract() == 0. || i >= CONFIGURATION.fix {
                break;
            }
            fract = fract.fract();
        }
        let ans = format!("{:>10}.{}", obase_int, obase_fract);
        println!("{}", thousand_sep(ans));
    }
}
