/* Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 * Refer to LICENCE for more information.
 * */

use std::fmt;
use std::iter::ExactSizeIterator;

use crate::lex;

/// Math related errors.
#[derive(Debug, PartialEq, Eq)]
pub enum MathError {
    DivideByZero,
    OutOfBounds,
    UnknownBase,
    TooLarge,
}

/// Generic calculation errors.
#[derive(Debug, PartialEq, Eq)]
pub enum CalcError {
    Math(MathError),
    Syntax(String),
    Parser(String),
    Help,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::Math(math_err) => match math_err {
                MathError::DivideByZero => write!(f, "Math Error: Divide by zero error!"),
                MathError::OutOfBounds => write!(f, "Domain Error: Out of bounds!"),
                MathError::UnknownBase => write!(f, "Base too large! Accepted ranges: 0 - 36"),
                MathError::TooLarge => {
                    write!(f, "Error: to large to process! Max value: {}", f64::MAX)
                }
            },
            CalcError::Syntax(details) => write!(f, "Syntax Error: {}", details),
            CalcError::Parser(details) => write!(f, "Parser Error: {}", details),
            CalcError::Help => {
                // calculate max width but ideally this should be calculated once
                // TODO remove terminal_size from lib dependency
                let mut max_width = 79; // capped at 79
                if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
                    if (w as usize) < max_width {
                        max_width = w as usize;
                    }
                }
                let operators: Vec<_> = lex::OPERATORS.keys().map(|c| c.to_string()).collect();
                write!(
                    f,
                    "Constants\n{}\nFunctions\n{}\nOperators\n{}\n",
                    blocks(max_width, lex::CONSTANTS.keys().cloned()),
                    blocks(max_width, lex::FUNCTIONS.keys().cloned()),
                    operators.join(" ")
                )
            }
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
