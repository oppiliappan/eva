/* Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 * Refer to LICENCE for more information.
 * */

use std::iter::ExactSizeIterator;

use crate::lex;

#[derive(Debug)]
pub enum CalcError {
    Math(Math),
    Syntax(String),
    Parser(String),
    Help,
}

#[derive(Debug)]
pub enum Math {
    DivideByZero,
    OutOfBounds,
    UnknownBase,
}

pub fn handler(e: CalcError) -> String {
    match e {
        CalcError::Math(math_err) => match math_err {
            Math::DivideByZero => "Math Error: Divide by zero error!".to_string(),
            Math::OutOfBounds => "Domain Error: Out of bounds!".to_string(),
            Math::UnknownBase => "Base too large! Accepted ranges: 0 - 36".to_string(),
        },
        CalcError::Syntax(details) => format!("Syntax Error: {}", details),
        CalcError::Parser(details) => format!("Parser Error: {}", details),
        CalcError::Help => format!(
            "Constants\n{}\nFunctions\n{}\nOperators\n{}\n",
            blocks(lex::CONSTANTS.keys().cloned()),
            blocks(lex::FUNCTIONS.keys().cloned()),
            {
                let l: Vec<_> = lex::OPERATORS.keys().map(|c| c.to_string()).collect();
                l.join(" ")
            }
        ),
    }
}

/// Convert iterator into strings of chunks of 8 right padded with space.
fn blocks(mut iter: impl Iterator<Item = &'static str> + ExactSizeIterator) -> String {
    // calculate max width but ideally this should be calculated once
    let mut max_width = 79; // capped at 79
    if let Ok(s) = std::env::var("COLUMNS") {
        if let Ok(n) = s.parse() {
            if n < max_width {
                max_width = n;
            }
        }
    }

    // multiply by eight since we are formatting it into chunks of 8
    let items_per_line = max_width / 8;
    let full_bytes = (iter.len() - iter.len() % items_per_line) * 8;
    let part_bytes = iter.len() % items_per_line * 8; // leftovers
    let n_newlines = iter.len() / items_per_line + if part_bytes > 0 { 1 } else { 0 };
    let mut s = String::with_capacity(full_bytes + part_bytes + n_newlines);
    for _ in 0..n_newlines {
        for item in iter.by_ref().take(items_per_line) {
            s.extend(format!("{:>8}", item).chars());
        }
        s.push('\n');
    }
    debug_assert_eq!(s.capacity(), s.len()); // check capacity calculation
    s
}
