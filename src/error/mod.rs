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
}

pub fn handler(e: CalcError) -> String {
    match e {
        CalcError::Math(math_err) => {
            match math_err {
                Math::DivideByZero => format!("Math Error: Divide by zero error!"),
                Math::OutOfBounds => format!("Domain Error: Out of bounds!")
            }
        },
        CalcError::Syntax(details) => {
            format!("Syntax Error: {}", details)
        },
        CalcError::Parser(details) => {
            format!("Parser Error: {}", details)
        }
    }
}
