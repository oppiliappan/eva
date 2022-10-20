/* Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 * Refer to LICENCE for more information.
 * */

use std::iter::ExactSizeIterator;

use crate::lex;

#[derive(Debug, PartialEq)]
pub enum CalcError {
    Math(Math),
    Syntax(String),
    Parser(String),
    Help,
}

#[derive(Debug, PartialEq)]
pub enum Math {
    DivideByZero,
    OutOfBounds,
    UnknownBase,
    TooLarge,
}

pub fn handler(e: CalcError) -> String {
    match e {
        CalcError::Math(math_err) => match math_err {
            Math::DivideByZero => "Math Error: Divide by zero error!".to_string(),
            Math::OutOfBounds => "Domain Error: Out of bounds!".to_string(),
            Math::UnknownBase => "Base too large! Accepted ranges: 0 - 36".to_string(),
            Math::TooLarge => {
                "Error: to large to process! Max value: ".to_string() + &f64::MAX.to_string()
            }
        },
        CalcError::Syntax(details) => format!("Syntax Error: {}", details),
        CalcError::Parser(details) => format!("Parser Error: {}", details),
        CalcError::Help => {
            // calculate max width but ideally this should be calculated once
            let mut max_width = 79; // capped at 79
            if let Some((w, _)) = term_size::dimensions() {
                if w < max_width {
                    max_width = w;
                }
            }
            let operators: Vec<_> = lex::OPERATORS.keys().map(|c| c.to_string()).collect();
            format!(
                "Constants\n{}\nFunctions\n{}\nOperators\n{}\n",
                blocks(max_width, lex::CONSTANTS.keys().cloned()),
                blocks(max_width, lex::FUNCTIONS.keys().cloned()),
                operators.join(" ")
            )
        }
    }
}

/// Convert iterator into strings of chunks of 8 right padded with space.
fn blocks(
    max_width: usize,
    mut iter: impl Iterator<Item = &'static str> + ExactSizeIterator,
) -> String {
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
