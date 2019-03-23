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

pub fn handler(e: CalcError) {
    match e {
        CalcError::Math(math_err) => {
            match math_err {
                Math::DivideByZero => println!("Math Error: Divide by zero error!"),
                Math::OutOfBounds => println!("Domain Error: Out of bounds!")
            }
        },
        CalcError::Syntax(details) => {
            println!("Syntax Error: {}", details);
        },
        CalcError::Parser(details) => {
            println!("Parser Error: {}", details);
        }
    }
}
