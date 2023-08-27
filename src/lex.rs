/* Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 * Refer to LICENCE for more information.
 * */

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;

use crate::error::{CalcError, MathError};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Operator {
    token: char,
    pub operation: fn(f64, f64) -> f64,
    pub precedence: u8,
    pub is_left_associative: bool,
}

impl Operator {
    pub fn operate(self, x: f64, y: f64) -> Result<f64, CalcError> {
        if self.token == '/' && y == 0. {
            return Err(CalcError::Math(MathError::DivideByZero));
        } else if self.token == '!' && (x < 0.0 || x.fract() != 0.0) {
            return Err(CalcError::Math(MathError::OutOfBounds));
        } else if self.token == '!' && x == 0.0 {
            // Must return 1 manually as 0..=n where n is 0.0 doesn't work AFAIK.
            return Ok(1.0);
        }
        let result = (self.operation)(x, y);
        if !result.is_finite() {
            Err(CalcError::Math(MathError::TooLarge))
        } else {
            Ok(result)
        }
    }
}

#[derive(Clone)]
pub enum Relation {
    N1(fn(&FunctionContext, f64) -> f64),
    N2(fn(&FunctionContext, f64, f64) -> f64),
}

#[derive(Clone)]
pub struct Function {
    token: &'static str,
    relation: Relation,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("fn").field("token", &self.token).finish()
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FunctionContext {
    pub angle_unit: AngleUnit,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AngleUnit {
    #[default]
    Degree,
    Radian,
    Gradian,
}

impl Function {
    pub fn apply(self, ctx: &FunctionContext, args: &[f64]) -> Result<f64, CalcError> {
        let result = match self.relation {
            Relation::N1(func) => (func)(ctx, args[0]),
            Relation::N2(func) => (func)(ctx, args[0], args[1]),
        };
        if result.is_finite() {
            Ok(result)
        } else {
            Err(CalcError::Math(MathError::OutOfBounds))
        }
    }
    pub fn arity(&self) -> usize {
        match self.relation {
            Relation::N1(_) => 1,
            Relation::N2(_) => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(Operator),
    Num(f64),
    Function(Function),
    LParen,
    RParen,
    Comma,
}

impl Token {
    fn from_op(
        token: char,
        operation: fn(f64, f64) -> f64,
        precedence: u8,
        is_left_associative: bool,
    ) -> Token {
        Token::Operator(Operator {
            token,
            operation,
            precedence,
            is_left_associative,
        })
    }
}

pub static CONSTANTS: Lazy<HashMap<&str, Token>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("e", Token::Num(std::f64::consts::E));
    m.insert("pi", Token::Num(std::f64::consts::PI));
    m
});

pub static FUNCTIONS: Lazy<HashMap<&str, Token>> = Lazy::new(|| {
    use Relation::*;
    fn add_fn(map: &mut HashMap<&str, Token>, token: &'static str, relation: Relation) {
        let func = Token::Function(Function { token, relation });
        map.insert(token, func);
    }
    let mut m = HashMap::new();
    add_fn(&mut m, "sin", N1(|ctx, x| rad(ctx, x).sin()));
    add_fn(&mut m, "cos", N1(|ctx, x| rad(ctx, x).cos()));
    add_fn(&mut m, "tan", N1(|ctx, x| rad(ctx, x).tan()));
    add_fn(&mut m, "csc", N1(|ctx, x| rad(ctx, x).sin().recip()));
    add_fn(&mut m, "sec", N1(|ctx, x| rad(ctx, x).cos().recip()));
    add_fn(&mut m, "cot", N1(|ctx, x| rad(ctx, x).tan().recip()));
    add_fn(&mut m, "sinh", N1(|_ctx, x| x.sinh()));
    add_fn(&mut m, "cosh", N1(|_ctx, x| x.cosh()));
    add_fn(&mut m, "tanh", N1(|_ctx, x| x.tanh()));
    add_fn(&mut m, "ln", N1(|_ctx, x| x.ln()));
    add_fn(&mut m, "log2", N1(|_ctx, x| x.log2()));
    add_fn(&mut m, "log10", N1(|_ctx, x| x.log10()));
    add_fn(&mut m, "sqrt", N1(|_ctx, x| x.sqrt()));
    add_fn(&mut m, "ceil", N1(|_ctx, x| x.ceil()));
    add_fn(&mut m, "floor", N1(|_ctx, x| x.floor()));
    add_fn(&mut m, "rad", N1(|_ctx, x| x.to_radians()));
    add_fn(&mut m, "deg", N1(|_ctx, x| x.to_degrees()));
    add_fn(&mut m, "abs", N1(|_ctx, x| x.abs()));
    add_fn(&mut m, "asin", N1(|_ctx, x| x.asin()));
    add_fn(&mut m, "acos", N1(|_ctx, x| x.acos()));
    add_fn(&mut m, "atan", N1(|_ctx, x| x.atan()));
    add_fn(&mut m, "acsc", N1(|_ctx, x| (1. / x).asin()));
    add_fn(&mut m, "asec", N1(|_ctx, x| (1. / x).acos()));
    add_fn(&mut m, "acot", N1(|_ctx, x| (1. / x).atan()));
    add_fn(&mut m, "exp", N1(|_ctx, x| x.exp()));
    add_fn(&mut m, "exp2", N1(|_ctx, x| x.exp2()));
    add_fn(&mut m, "round", N1(|_ctx, x| x.round()));
    add_fn(&mut m, "log", N2(|_ctx, x, y| x.log(y)));
    add_fn(&mut m, "nroot", N2(|_ctx, x, y| x.powf(1. / y)));
    m
});

pub static OPERATORS: Lazy<HashMap<char, Token>> = Lazy::new(|| {
    fn add_op(
        map: &mut HashMap<char, Token>,
        token: char,
        operation: fn(f64, f64) -> f64,
        precedence: u8,
        is_left_associative: bool,
    ) {
        let op = Token::from_op(token, operation, precedence, is_left_associative);
        map.insert(token, op);
    }
    let mut m = HashMap::new();
    add_op(&mut m, '+', |x, y| x + y, 2, true);
    add_op(&mut m, '-', |x, y| x - y, 2, true);
    add_op(&mut m, '*', |x, y| x * y, 3, true);
    add_op(&mut m, '/', |x, y| x / y, 3, true);
    add_op(&mut m, '%', |x, y| x % y, 3, true);
    add_op(&mut m, '^', |x, y| x.powf(y), 4, false);
    add_op(&mut m, '!', |x, _| factorial(x), 4, true);
    m
});

fn factorial(n: f64) -> f64 {
    let answer = (1..=n.round() as u128).map(|u| u as f64).product::<f64>() as f64;
    if answer == 0.0 {
        return f64::INFINITY;
    }
    answer
}

pub(crate) fn lexer(input: &str, prev_ans: Option<f64>) -> Result<Vec<Token>, CalcError> {
    let mut num_vec: String = String::new();
    let mut char_vec: String = String::new();
    let mut result: Vec<Token> = vec![];
    let mut last_char_is_op = true;

    let mut chars = input.chars().peekable();
    while let Some(mut letter) = chars.next() {
        match letter {
            '0'..='9' | '.' => {
                if !char_vec.is_empty() {
                    if FUNCTIONS.get(&char_vec[..]).is_some() {
                        char_vec.push(letter);
                        if FUNCTIONS.get(&char_vec[..]).is_none()
                            && !FUNCTIONS.keys().any(|k| k.starts_with(&char_vec))
                        {
                            return Err(CalcError::Syntax(format!(
                                "Function '{}' expected parentheses",
                                &char_vec[..char_vec.chars().count() - 1]
                            )));
                        }
                    } else if CONSTANTS.get(&char_vec[..]).is_some() {
                        result.push(CONSTANTS.get(&char_vec[..]).unwrap().clone());
                        result.push(OPERATORS.get(&'*').unwrap().clone());
                        char_vec.clear();
                        num_vec.push(letter);
                        last_char_is_op = false;
                    } else {
                        char_vec.push(letter);
                        if FUNCTIONS.get(&char_vec[..]).is_none() {
                            return Err(CalcError::Syntax(format!(
                                "Unexpected character '{}'",
                                char_vec
                            )));
                        }
                    }
                } else {
                    num_vec.push(letter);
                    last_char_is_op = false;
                }
            }
            '_' => {
                if prev_ans.is_none() {
                    return Err(CalcError::Syntax("No previous answer!".into()));
                }
                if !char_vec.is_empty() {
                    if FUNCTIONS.get(&char_vec[..]).is_some() {
                        return Err(CalcError::Syntax(format!(
                            "Function '{}' expected parentheses",
                            char_vec
                        )));
                    } else {
                        return Err(CalcError::Syntax(format!(
                            "Unexpected character '{}'",
                            char_vec
                        )));
                    }
                }
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    result.push(OPERATORS.get(&'*').unwrap().clone());
                    num_vec.clear();
                }
                last_char_is_op = false;
                result.push(Token::Num(prev_ans.unwrap()));
            }
            'a'..='z' | 'A'..='Z' => {
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    result.push(OPERATORS.get(&'*').unwrap().clone());
                    num_vec.clear();
                }
                char_vec.push(letter);
                last_char_is_op = false;
            }
            '+' | '-' => {
                let op_token = OPERATORS.get(&letter).unwrap().clone();
                let parse_num = num_vec.parse::<f64>().ok();
                if !last_char_is_op {
                    if let Some(x) = parse_num {
                        result.push(Token::Num(x));
                        num_vec.clear();
                        last_char_is_op = true;
                    } else if let Some(token) = CONSTANTS.get(&char_vec[..]) {
                        result.push(token.clone());
                        char_vec.clear();
                        last_char_is_op = true;
                    }
                    result.push(op_token);
                } else if last_char_is_op {
                    result.push(Token::LParen);
                    result.push(Token::Num(
                        (letter.to_string() + "1").parse::<f64>().unwrap(),
                    ));
                    result.push(Token::RParen);
                    result.push(Token::from_op('*', |x, y| x * y, 10, true));
                }
            }
            '/' | '*' | '%' | '^' | '!' => {
                drain_stack(&mut num_vec, &mut char_vec, &mut result);
                if letter == '*' && chars.peek() == Some(&'*') {
                    // Accept `**` operator as meaning `^` (exponentation).
                    let _ = chars.next();
                    letter = '^';
                }
                let operator_token: Token = OPERATORS.get(&letter).unwrap().clone();
                result.push(operator_token);
                last_char_is_op = true;
                if letter == '!' {
                    result.push(Token::Num(1.));
                    last_char_is_op = false;
                }
            }
            '(' => {
                if !char_vec.is_empty() {
                    if let Some(res) = FUNCTIONS.get(&char_vec[..]) {
                        result.push(res.clone());
                    } else {
                        return Err(CalcError::Syntax(format!(
                            "Unknown function '{}'",
                            char_vec
                        )));
                    }
                    char_vec.clear();
                } else {
                    let parse_num = num_vec.parse::<f64>().ok();
                    if let Some(x) = parse_num {
                        result.push(Token::Num(x));
                        result.push(OPERATORS.get(&'*').unwrap().clone());
                        num_vec.clear();
                    }
                }

                if let Some(Token::RParen) = result.last() {
                    result.push(OPERATORS.get(&'*').unwrap().clone());
                }
                result.push(Token::LParen);
                last_char_is_op = true;
            }
            ',' => {
                drain_stack(&mut num_vec, &mut char_vec, &mut result);
                result.push(Token::Comma);
            }
            ')' => {
                drain_stack(&mut num_vec, &mut char_vec, &mut result);
                result.push(Token::RParen);
                last_char_is_op = false;
            }
            ' ' => {}
            _ => return Err(CalcError::Syntax(format!("Unexpected token: '{}'", letter))),
        }
    }
    // println!("{:?}", result);
    drain_stack(&mut num_vec, &mut char_vec, &mut result);
    Ok(result)
}

fn drain_stack(num_vec: &mut String, char_vec: &mut String, result: &mut Vec<Token>) {
    let parse_num = num_vec.parse::<f64>().ok();
    if let Some(x) = parse_num {
        result.push(Token::Num(x));
        num_vec.clear();
    } else if let Some(token) = CONSTANTS.get(&char_vec[..]) {
        result.push(token.clone());
        char_vec.clear();
    }
}

/// Convert to radian if radian_mode is enabled.
fn rad(ctx: &FunctionContext, x: f64) -> f64 {
    // TODO gradian
    if ctx.angle_unit == AngleUnit::Radian {
        x
    } else {
        x.to_radians()
    }
}
