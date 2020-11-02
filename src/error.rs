/* Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 * Refer to LICENCE for more information.
 * */

#[derive(Debug)]
pub enum CalcError {
    Math(Math),
    Syntax(String),
    Parser(String),
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
    }
}
